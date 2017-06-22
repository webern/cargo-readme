//! Read crate information from `Cargo.toml`

use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

#[derive(Clone, Deserialize)]
pub struct Cargo {
    pub package: CargoPackage,
    pub lib: Option<CargoLib>,
    pub bin: Option<Vec<CargoLib>>,
}

#[derive(Clone, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub license: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct CargoLib {
    pub path: String,
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
