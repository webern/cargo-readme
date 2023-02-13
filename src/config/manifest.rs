//! Read crate information from `Cargo.toml`

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use toml;

use super::badges;

/// Try to get manifest info from Cargo.toml
pub fn get_manifest(project_root: &Path) -> Result<Manifest, String> {
    let cargo_toml = std::fs::read_to_string(project_root.join("Cargo.toml"))
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml).map_err(|e| format!("{}", e))?;

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(project_root.join("Cargo.toml"))
        .exec()
        .map_err(|e| e.to_string())?;
    let package = metadata.root_package().unwrap();

    let manifest = Manifest::new(package, cargo_toml.badges);

    Ok(manifest)
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
    fn new(
        cargo_toml: &cargo_metadata::Package,
        badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ) -> Manifest {
        let lib = cargo_toml.targets.iter().find(|t| t.is_lib());
        let bins = cargo_toml.targets.iter().filter(|t| t.is_bin());

        Manifest {
            name: cargo_toml.name.clone(),
            license: cargo_toml.license.clone(),
            lib: lib.map(ManifestLib::from_cargo_toml),
            bin: bins.map(ManifestLib::from_cargo_toml).collect(),
            badges: badges.map(process_badges).unwrap_or_default(),
            version: cargo_toml.version.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ManifestLib {
    pub path: PathBuf,
    pub doc: bool,
}

impl ManifestLib {
    fn from_cargo_toml(lib: &cargo_metadata::Target) -> Self {
        ManifestLib {
            path: lib.src_path.clone().into(),
            doc: lib.doc,
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
            _ => return None,
        })
        .collect();

    b.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    b.into_iter().map(|(_, badge)| badge).collect()
}

/// Cargo.toml crate information
#[derive(Clone, Deserialize)]
struct CargoToml {
    pub badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
