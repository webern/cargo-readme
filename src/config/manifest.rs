//! Read crate information from `Cargo.toml`

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::badges;

/// Try to get manifest info from Cargo.toml
pub fn get_manifest(project_root: &Path) -> Result<Manifest, String> {
    let cargo_toml_path = project_root.join("Cargo.toml");

    let manifest = cargo_toml::Manifest::<toml::Value>::from_path(&cargo_toml_path)
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;

    let raw_toml_text = std::fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;
    let raw_toml: RawBadges = toml::from_str(&raw_toml_text).map_err(|e| format!("{}", e))?;

    Manifest::try_new(manifest, raw_toml.badges)
}

/// Error message for a field that could not be resolved through workspace inheritance
fn workspace_inherit_err(field: &str) -> String {
    format!(
        "Could not resolve `{field}`: the crate sets `{field}.workspace = true`, but no \
         workspace root defines `{field}` under `[workspace.package]` (or the workspace \
         root could not be found)"
    )
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
        manifest: cargo_toml::Manifest<toml::Value>,
        badges_raw: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ) -> Result<Manifest, String> {
        let package = manifest
            .package
            .as_ref()
            .ok_or_else(|| "Missing [package] section in Cargo.toml".to_string())?;

        let name = package.name.clone();

        let license = package
            .license
            .as_ref()
            .map(|l| l.get().map(|s| s.to_owned()))
            .transpose()
            .map_err(|_| workspace_inherit_err("license"))?;

        let lib = manifest
            .lib
            .as_ref()
            .map(ManifestLib::from_product)
            .transpose()?;

        let bin = manifest
            .bin
            .iter()
            .map(ManifestLib::from_product)
            .collect::<Result<Vec<_>, _>>()?;

        let badges = badges_raw.map(process_badges).unwrap_or_default();

        let version = package
            .version
            .get()
            .map_err(|_| workspace_inherit_err("version"))?
            .to_string();

        Ok(Manifest {
            name,
            license,
            lib,
            bin,
            badges,
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
    fn from_product(product: &cargo_toml::Product) -> Result<Self, String> {
        let path = product
            .path
            .as_ref()
            .ok_or_else(|| "Missing path for product".to_string())?;

        Ok(ManifestLib {
            path: PathBuf::from(path),
            doc: product.doc,
        })
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

    b.sort_unstable_by_key(|a| a.0);
    b.into_iter().map(|(_, badge)| badge).collect()
}

/// Raw badges extraction from TOML
#[derive(Clone, Deserialize)]
struct RawBadges {
    pub badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
