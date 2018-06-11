//! Read crate information from `Cargo.toml`

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;
use percent_encoding as pe;

const BADGE_BRANCH_DEFAULT: &str = "master";
const BADGE_SERVICE_DEFAULT: &str = "github";

/// Try to get manifest info from Cargo.toml
pub fn get_manifest(project_root: &Path) -> Result<Manifest, String> {
    let mut cargo_toml = File::open(project_root.join("Cargo.toml"))
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;

    let buf = {
        let mut buf = String::new();
        cargo_toml.read_to_string(&mut buf)
            .map_err(|e| format!("{}", e))?;
        buf
    };

    let cargo_toml: CargoToml = toml::from_str(&buf)
        .map_err(|e| format!("{}", e))?;

    let manifest = Manifest::new(cargo_toml);

    Ok(manifest)
}

#[derive(Debug)]
pub struct Manifest {
    pub name: String,
    pub license: Option<String>,
    pub lib: Option<ManifestLib>,
    pub bin: Vec<ManifestLib>,
    pub badges: Vec<String>,
}

impl Manifest {
    fn new(cargo_toml: CargoToml) -> Manifest {
        Manifest {
            name: cargo_toml.package.name,
            license: cargo_toml.package.license,
            lib: cargo_toml.lib.map(|lib| ManifestLib::from_cargo_toml(lib)),
            bin: cargo_toml.bin.map(|bin_vec| {
                bin_vec.into_iter().map(|bin| ManifestLib::from_cargo_toml(bin)).collect()
            }).unwrap_or_default(),
            badges: cargo_toml.badges
                .map(|b| process_badges(b))
                .unwrap_or_default()
        }
    }
}

#[derive(Debug)]
pub struct ManifestLib {
    pub path: PathBuf,
    pub doc: bool,
}

impl ManifestLib {
    fn from_cargo_toml(lib: CargoTomlLib) -> Self {
        ManifestLib {
            path: PathBuf::from(lib.path),
            doc: lib.doc.unwrap_or(true)
        }
    }
}

fn badge_filter_map((name, attrs): (String, BTreeMap<String, String>)) -> Option<(u16, String)> {
    let mut order = 0;
    let badge = match name.as_ref() {
        "appveyor" => {
            order = 1;

            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
            let service = attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT);

            format!(
                "[![Build Status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)](https://ci.appveyor.com/project/{repo}/branch/{branch})",
                repo=repo, branch=branch, service=service
            )
        }
        "circle-ci" => {
            order = 2;

            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
            let service = badge_service_short_name(
                attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT)
            );

            format!(
                "[![Build Status](https://circleci.com/{service}/{repo}/tree/{branch}.svg?style=svg)](https://circleci.com/{service}/{repo}/cargo-readme/tree/{branch})",
                repo=repo, service=service, branch=percent_encode(branch)
            )
        }
        "gitlab" => {
            order = 3;

            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);

            format!(
                "[![Build Status](https://gitlab.com/{repo}/badges/{branch}/build.svg)](https://gitlab.com/{repo}/commits/master)",
                repo=repo, branch=percent_encode(branch)
            )
        }
        "travis-ci" => {
            order = 4;

            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);

            format!(
                "[![Build Status](https://travis-ci.org/{repo}.svg?branch={branch})](https://travis-ci.org/{repo})",
                repo=repo, branch=percent_encode(branch)
            )
        }
        "codecov" => {
            order = 5;
            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
            let service = badge_service_short_name(
                attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT)
            );

            format!(
                "[![Coverage Status](https://codecov.io/{service}/{repo}/branch/{branch}/graph/badge.svg)](https://codecov.io/{service}/{repo})",
                repo=repo, branch=percent_encode(branch), service=service
            )
        }
        "coveralls" => {
            order = 6;

            let repo = &attrs["repository"];
            let branch = attrs.get("branch").map(|i| i.as_ref()).unwrap_or(BADGE_BRANCH_DEFAULT);
            let service = attrs.get("service").map(|i| i.as_ref()).unwrap_or(BADGE_SERVICE_DEFAULT);

            format!(
                "[![Coverage Status](https://coveralls.io/repos/{service}/{repo}/badge.svg?branch=branch)](https://coveralls.io/{service}/{repo}?branch={branch})",
                repo=repo, branch=percent_encode(branch), service=service
            )
        }
        "is-it-maintained-issue-resolution" => {
            order = 7;

            format!(
                "[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/{repo}.svg)](http://isitmaintained.com/project/{repo} \"Average time to resolve an issue\")",
                repo=attrs["repository"]
            )
        }
        "is-it-maintained-open-issues" => {
            order = 8;

            format!(
                "[![Percentage of issues still open](http://isitmaintained.com/badge/open/{repo}.svg)](http://isitmaintained.com/project/{repo} \"Percentage of issues still open\")",
                repo=attrs["repository"]
            )
        }
        _ => {
            return None;
        }
    };

    Some((order, badge))
}

fn process_badges(badges: BTreeMap<String, BTreeMap<String, String>>) -> Vec<String> {
    let mut b: Vec<(u16, _)> = badges.into_iter()
        .filter_map(badge_filter_map).collect();

    b.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    b.into_iter().map(|(_, badge)| badge).collect()
}

fn percent_encode(input: &str) -> pe::PercentEncode<pe::PATH_SEGMENT_ENCODE_SET> {
    pe::utf8_percent_encode(input, pe::PATH_SEGMENT_ENCODE_SET)
}

fn badge_service_short_name(service: &str) -> &'static str {
    match service {
        "github" => "gh",
        "bitbucket" => "bb",
        "gitlab" => "gl",
        _ => "gh",
    }
}

/// Cargo.toml crate information
#[derive(Clone, Deserialize)]
struct CargoToml {
    pub package: CargoTomlPackage,
    pub lib: Option<CargoTomlLib>,
    pub bin: Option<Vec<CargoTomlLib>>,
    pub badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
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
