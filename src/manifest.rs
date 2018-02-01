//! Read crate information from `Cargo.toml`

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

pub struct CrateInfo {
    pub name: String,
    pub license: Option<String>,
    pub lib: Option<(PathBuf, Option<bool>)>,
    pub bin: Vec<(String, Option<bool>)>,
    pub badges: Vec<String>,
}

impl CrateInfo {
    fn new(manifest: Manifest) -> CrateInfo {
        CrateInfo {
            name: manifest.package.name,
            license: manifest.package.license,
            lib: manifest.lib.map(|lib| (PathBuf::from(lib.path), lib.doc)),
            bin: manifest.bin.map(|bin_vec| bin_vec.into_iter().map(|bin| (bin.path, bin.doc)).collect()).unwrap_or_default(),
            badges: process_badges(manifest.badges)
        }
    }
}

fn process_badges(badges: BTreeMap<String, BTreeMap<String, String>>) -> Vec<String> {
    badges.into_iter().map(|(name, attrs)| process_badge(name, attrs)).collect()
}

fn process_badge(name: String, attrs: BTreeMap<String, String>) -> String {
    match name.as_ref() {
        "appveyor" => {
            // TODO: handle error properly
            let repo = attrs.get("repository").unwrap();
            // TODO: use constants instead of magic values
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or("master");
            let service = attrs.get("service").map(|i| i.as_ref()).unwrap_or("github");
            format!("[![Build status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)](https://ci.appveyor.com/project/{repo}/branch/{branch})",
                repo=repo, branch=branch, service=service)
        }
        // "circle-ci" => {
            // [![CircleCI](https://circleci.com/gh/livioribeiro/cargo-readme.svg?style=svg)](https://circleci.com/gh/livioribeiro/cargo-readme)
            // [![CircleCI](https://circleci.com/gh/livioribeiro/cargo-readme/tree/master.svg?style=svg)](https://circleci.com/gh/livioribeiro/cargo-readme/tree/master)
        // }
        _ => unimplemented!()
    }
}

/// Cargo.toml crate information
#[derive(Clone, Deserialize)]
struct Manifest {
    pub package: ManifestPackage,
    pub lib: Option<ManifestLib>,
    pub bin: Option<Vec<ManifestLib>>,
    pub badges: BTreeMap<String, BTreeMap<String, String>>,
}

/// Cargo.toml crate package information
#[derive(Clone, Deserialize)]
struct ManifestPackage {
    pub name: String,
    pub license: Option<String>,
}

/// Cargo.toml crate lib information
#[derive(Clone, Deserialize)]
struct ManifestLib {
    pub path: String,
    pub doc: Option<bool>,
}

/// Try to get crate info from Cargo.toml
pub fn get_crate_info(project_root: &Path) -> Result<CrateInfo, String> {
    let mut cargo_toml = File::open(project_root.join("Cargo.toml"))
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;

    let buf = {
        let mut buf = String::new();
        cargo_toml.read_to_string(&mut buf)
            .map_err(|e| format!("{}", e))?;
        buf
    };

    let manifest = toml::from_str(&buf)
        .map_err(|e| format!("{}", e))?;

    Ok(CrateInfo::new(manifest))
}
