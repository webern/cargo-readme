//! Read crate information from `Cargo.toml`

use std::path::{Path, PathBuf};

use super::badges;

/// Try to get manifest info from Cargo.toml
pub fn get_manifest(project_root: &Path) -> Result<Manifest, String> {
    Manifest::try_new(
        cargo_toml::Manifest::from_path(project_root.join("Cargo.toml"))
            .map_err(|e| format!("{}", e))?,
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
    fn try_new(manifest: cargo_toml::Manifest) -> Result<Manifest, String> {
        let package = manifest
            .package
            .ok_or_else(|| "Missing [package] section in Cargo.toml".to_string())?;

        let license = match package.license {
            Some(license) => match license.get() {
                Ok(license) => Some(license.to_owned()),
                Err(err) => return Err(format!("{:?}", err)),
            },
            None => None,
        };

        let lib = ManifestLib::from_cargo_toml(manifest.lib.as_ref());

        let bin = manifest
            .bin
            .iter()
            .filter_map(|bin| ManifestLib::from_cargo_toml(Some(bin)))
            .collect::<Vec<_>>();

        let badges = [
            manifest.badges.appveyor.map(badges::appveyor),
            manifest.badges.circle_ci.map(badges::circle_ci),
            manifest.badges.gitlab.map(badges::gitlab),
            #[allow(deprecated)]
            manifest.badges.travis_ci.map(badges::travis_ci),
            // TODO: support github
            // manifest.badges.github,
            manifest.badges.codecov.map(badges::codecov),
            manifest.badges.coveralls.map(badges::coveralls),
            manifest
                .badges
                .is_it_maintained_issue_resolution
                .map(badges::is_it_maintained_issue_resolution),
            manifest
                .badges
                .is_it_maintained_open_issues
                .map(badges::is_it_maintained_open_issues),
            badges::maintenance(manifest.badges.maintenance),
        ]
        .into_iter()
        .filter_map(|b| b)
        .collect::<Vec<_>>();

        let version = package
            .version
            .get()
            .map_err(|err| format!("{:?}", err))?
            .to_owned();

        Ok(Manifest {
            name: package.name,
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
    fn from_cargo_toml(product: Option<&cargo_toml::Product>) -> Option<Self> {
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
