//! Read crate information from `Cargo.toml`

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;
use percent_encoding as pe;

const BADGE_BRANCH_DEFAULT: &str = "master";
const BADGE_SERVICE_DEFAULT: &str = "github";

const BADGE_PROVIDERS: [&str; 8] = [
    "appveyor",
    "circle-ci",
    "gitlab",
    "travis-ci",
    "codecov",
    "coveralls",
    "is-it-maintained-issue-resolution",
    "is-it-maintained-open-issues",
];

pub struct Manifest {
    pub name: String,
    pub license: Option<String>,
    pub lib: Option<ManifestLib>,
    pub bin: Vec<ManifestLib>,
    pub badges: Vec<String>,
}

impl Manifest {
    fn new(manifest: CargoToml) -> Manifest {
        Manifest {
            name: manifest.package.name,
            license: manifest.package.license,
            lib: manifest.lib.map(|lib| ManifestLib::from_cargo_toml(lib)),
            bin: manifest.bin.map(|bin_vec| {
                bin_vec.into_iter().map(|bin| ManifestLib::from_cargo_toml(bin)).collect()
            }).unwrap_or_default(),
            badges: process_badges(manifest.badges)
        }
    }
}

pub struct ManifestLib {
    pub path: PathBuf,
    pub doc: bool,
}

impl ManifestLib {
    fn new (path: PathBuf, doc: bool) -> Self {
        ManifestLib {
            path: path,
            doc: doc
        }
    }

    fn from_cargo_toml(lib: CargoTomlLib) -> Self {
        ManifestLib {
            path: PathBuf::from(lib.path),
            doc: lib.doc.unwrap_or(true)
        }
    }
}

fn process_badges(badges: BTreeMap<String, BTreeMap<String, String>>) -> Vec<String> {
    badges.into_iter()
        .filter(|&(name, _)| BADGE_PROVIDERS.contains(&name.as_ref()))
        .map(|(name, attrs)| {
        match name.as_ref() {
            "appveyor" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
                let service = attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT);

                format!(
                    "[![Build status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)](https://ci.appveyor.com/project/{repo}/branch/{branch})",
                    repo=repo, branch=branch, service=percent_encode(service))
            }
            "circle-ci" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
                let service = badge_service_short_name(
                    attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT)
                );

                format!(
                    "[![Build status](https://circleci.com/{service}/{repo}/tree/{branch}.svg?style=svg)](https://circleci.com/{service}/{repo}/cargo-readme/tree/{branch})",
                    repo=repo, service=service, branch=percent_encode(branch))
            }
            "gitlab" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);

                format!(
                    "[![Build status](https://gitlab.com/{repo}/badges/{branch}/build.svg)](https://gitlab.com/{repo}/commits/master)",
                    repo=repo, branch=percent_encode(branch))
            }
            "travis-ci" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);

                format!(
                    "[![Build Status](https://travis-ci.org/{repo}.svg?branch={branch})](https://travis-ci.org/{repo})",
                    repo=repo, branch=percent_encode(branch))
            }
            "codecov" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
                let service = badge_service_short_name(
                    attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT)
                );

                format!(
                    "[![Coverage Status](https://codecov.io/{service}/{repo}/branch/{branch}/graph/badge.svg)](https://codecov.io/{service}/{repo})",
                    repo=repo, branch=percent_encode(branch), service=service)
            }
            "coveralls" => {
                let repo = attrs["repository"];
                let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
                let service = attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT);

                format!(
                    "[![Coverage Status](https://coveralls.io/repos/{service}/{repo}/badge.svg?branch=branch)](https://coveralls.io/{service}/{repo}?branch={branch})",
                    repo=repo, branch=percent_encode(branch), service=service)
            }
            "is-it-maintained-issue-resolution" => {
                format!(
                    "[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/{repo}.svg)](http://isitmaintained.com/project/{repo} \"Average time to resolve an issue\")",
                    repo=attrs["repository"])
            }
            "is-it-maintained-open-issues" => {
                format!(
                    "[![Percentage of issues still open](http://isitmaintained.com/badge/open/{repo}.svg)](http://isitmaintained.com/project/{repo} \"Percentage of issues still open\")",
                    repo=attrs["repository"])
            }
        }
    }).collect()
}

fn percent_encode(input: &str) -> pe::PercentEncode<pe::PATH_SEGMENT_ENCODE_SET> {
    pe::utf8_percent_encode(input, pe::PATH_SEGMENT_ENCODE_SET)
}

fn badge_service_short_name(service: &str) -> &'static str {
    match service {
        "github" => "gh",
        "bitbucket" => "bb",
        "gitlab" => "gl",
    }
}

/// Cargo.toml crate information
#[derive(Clone, Deserialize)]
struct CargoToml {
    pub package: CargoTomlPackage,
    pub lib: Option<CargoTomlLib>,
    pub bin: Option<Vec<CargoTomlLib>>,
    pub badges: BTreeMap<String, BTreeMap<String, String>>,
}

/// Cargo.toml crate package information
#[derive(Clone, Deserialize)]
struct CargoTomlPackage {
    pub name: String,
    pub license: Option<String>,
}

/// Cargo.toml crate lib information
#[derive(Clone, Deserialize)]
struct CargoTomlLib {
    pub path: String,
    pub doc: Option<bool>,
}

/// Try to get crate info from Cargo.toml
pub fn get_crate_info(project_root: &Path) -> Result<Manifest, String> {
    let mut cargo_toml = File::open(project_root.join("Cargo.toml"))
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;

    let buf = {
        let mut buf = String::new();
        cargo_toml.read_to_string(&mut buf)
            .map_err(|e| format!("{}", e))?;
        buf
    };

    let cargo_toml = toml::from_str(&buf)
        .map_err(|e| format!("{}", e))?;

    Ok(Manifest::new(cargo_toml))
}
