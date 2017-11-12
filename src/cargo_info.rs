//! Read crate information from `Cargo.toml`

use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

pub const DEFAULT_LIB: &'static str = "src/lib.rs";

/// Cargo.toml crate information
#[derive(Clone, Deserialize)]
pub struct Cargo {
    pub package: CargoPackage,
    pub lib: Option<CargoLib>,
    pub bin: Option<Vec<CargoLib>>,
}

/// Cargo.toml crate package information
#[derive(Clone, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub license: Option<String>,
}

/// Cargo.toml crate lib information
#[derive(Clone, Deserialize)]
pub struct CargoLib {
    pub path: Option<String>,
}

impl CargoLib {
    pub fn path_or_default(&self) -> &str {
        self.path
            .as_ref()
            .map(|p| p.as_ref())
            .unwrap_or(DEFAULT_LIB)
    }
}

/// Try to get crate name and license from Cargo.toml
pub fn get_cargo_info(project_root: &Path) -> Result<Cargo, String> {
    let mut cargo_toml = match File::open(project_root.join("Cargo.toml")) {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not read Cargo.toml: {}", e)),
    };

    let mut buf = String::new();
    match cargo_toml.read_to_string(&mut buf) {
        Err(e) => return Err(format!("{}", e)),
        Ok(_) => {}
    }

    match toml::from_str(&buf) {
        Err(e) => return Err(format!("{}", e)),
        Ok(cargo) => Ok(cargo),
    }
}
