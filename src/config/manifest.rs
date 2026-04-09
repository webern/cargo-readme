//! Read crate information from `Cargo.toml`

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::badges;

/// Try to get manifest info from Cargo.toml
pub fn get_manifest(project_root: &Path) -> Result<Manifest, String> {
    let cargo_toml_path = project_root.join("Cargo.toml");

    // Use cargo_toml crate for workspace inheritance support
    let manifest = cargo_toml::Manifest::from_path(&cargo_toml_path)
        .map_err(|e| format!("{}", e))?;

    // Also parse raw TOML for badges (cargo_toml crate doesn't support all badge types)
    let raw_toml = std::fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;
    let raw: RawCargoToml = toml::from_str(&raw_toml).map_err(|e| format!("{}", e))?;

    Manifest::try_new(manifest, raw.badges)
}

#[derive(Debug)]
pub struct Manifest {
    pub name: String,
    pub license: Option<String>,
    pub lib: Option<ManifestLib>,
    pub bin: Vec<ManifestLib>,
    pub badges: Vec<String>,
    pub version: String,
}

impl Manifest {
    fn try_new(
        manifest: cargo_toml::Manifest,
        badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ) -> Result<Manifest, String> {
        let package = manifest
            .package
            .ok_or_else(|| "Missing [package] section in Cargo.toml".to_string())?;

        let license = match package.license {
            Some(license) => match license.get() {
                Ok(license) => Some(license.to_owned()),
                Err(_) => {
                    return Err(
                        "Could not resolve workspace-inherited license".to_string()
                    )
                }
            },
            None => None,
        };

        let lib = ManifestLib::from_product(manifest.lib.as_ref());

        let bin = manifest
            .bin
            .iter()
            .filter_map(|bin| ManifestLib::from_product(Some(bin)))
            .collect::<Vec<_>>();

        let version = package
            .version
            .get()
            .map_err(|_| "Could not resolve workspace-inherited version".to_string())?
            .to_owned();

        Ok(Manifest {
            name: package.name,
            license,
            lib,
            bin,
            badges: badges.map(process_badges).unwrap_or_default(),
            version,
        })
    }
}

#[derive(Debug)]
pub struct ManifestLib {
    pub path: PathBuf,
    pub doc: bool,
}

impl ManifestLib {
    fn from_product(product: Option<&cargo_toml::Product>) -> Option<Self> {
        if let Some(cargo_toml::Product {
            path: Some(path),
            doc,
            ..
        }) = product
        {
            Some(ManifestLib {
                path: path.into(),
                doc: *doc,
            })
        } else {
            None
        }
    }
}

fn process_badges(badges: BTreeMap<String, BTreeMap<String, String>>) -> Vec<String> {
    let mut b: Vec<(u16, _)> = badges
        .into_iter()
        .filter_map(|(name, attrs)| match name.as_ref() {
            "appveyor" => Some((0, badges::appveyor(attrs))),
            "circle-ci" => Some((1, badges::circle_ci(attrs))),
            "gitlab" => Some((2, badges::gitlab(attrs))),
            "travis-ci" => Some((3, badges::travis_ci(attrs))),
            "github" => Some((4, badges::github(attrs))),
            "codecov" => Some((5, badges::codecov(attrs))),
            "coveralls" => Some((6, badges::coveralls(attrs))),
            "is-it-maintained-issue-resolution" => {
                Some((7, badges::is_it_maintained_issue_resolution(attrs)))
            }
            "is-it-maintained-open-issues" => {
                Some((8, badges::is_it_maintained_open_issues(attrs)))
            }
            "maintenance" => Some((9, badges::maintenance(attrs))),
            _ => None,
        })
        .collect();

    b.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    b.into_iter().map(|(_, badge)| badge).collect()
}

/// Used for extracting badges from raw TOML (cargo_toml crate doesn't support all badge types)
#[derive(Deserialize)]
struct RawCargoToml {
    badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
