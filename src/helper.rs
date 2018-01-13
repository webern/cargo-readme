use std::env;
use std::io::{self, Write, ErrorKind};
use std::fs::File;
use std::path::{Path, PathBuf};

use cargo_info;

const DEFAULT_TEMPLATE: &'static str = "README.tpl";

/// Get the project root from given path or defaults to current directory
///
/// The given path is appended to the current directory if is a relative path, otherwise it is used
/// as is. If no path is given, the current directory is used.
/// A `Cargo.toml` file must be present is the root directory.
pub fn get_project_root(given_root: Option<&str>) -> Result<PathBuf, String> {
    let current_dir = env::current_dir().map_err(|e| format!("{}", e))?;
    let root = match given_root {
        Some(root) => {
            let root = Path::new(root);
            if root.is_absolute() {
                root.to_path_buf()
            } else {
                current_dir.join(root)
            }
        }
        None => current_dir,
    };

    if !root.join("Cargo.toml").is_file() {
        return Err(format!(
            "`{:?}` does not look like a Rust/Cargo project",
            root
        ));
    }

    Ok(root)
}

/// Get the source file from which the doc comments will be extracted
pub fn get_source(project_root: &Path, input: Option<&str>) -> Result<File, String> {
    match input {
        Some(input) => {
            let input = project_root.join(input);
            File::open(&input).map_err(|e| {
                format!("Could not open file '{}': {}", input.to_string_lossy(), e)
            })
        }
        None => find_entrypoint(&project_root),
    }
}

/// Get the destination file where the result will be output to
pub fn get_dest(project_root: &Path, output: Option<&str>) -> Result<Option<File>, String> {
    match output {
        Some(filename) => {
            let output = project_root.join(filename);
            File::create(&output).map(|f| Some(f)).map_err(|e| {
                format!(
                    "Could not create output file '{}': {}",
                    output.to_string_lossy(),
                    e
                )
            })
        }
        None => Ok(None),
    }
}

/// Get the template file that will be used to render the output
pub fn get_template_file(project_root: &Path, template: Option<&str>) -> Result<Option<File>, String> {
    match template {
        // template path was given, try to read it
        Some(template) => {
            let template = project_root.join(template);
            File::open(&template).map(|f| Some(f)).map_err(|e| {
                format!(
                    "Could not open template file '{}': {}",
                    template.to_string_lossy(),
                    e
                )
            })
        }
        // try to read the defautl template file
        None => {
            let template = project_root.join(DEFAULT_TEMPLATE);
            match File::open(&template) {
                Ok(file) => Ok(Some(file)),
                // do not generate an error on file not found
                Err(ref e) if e.kind() != ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open template file '{}': {}",
                        DEFAULT_TEMPLATE,
                        e
                    ))
                }
                // default template not found, return `None`
                _ => Ok(None),
            }
        }
    }
}

/// Write result to output, either stdout or destination file
pub fn write_output(dest: &mut Option<File>, readme: String) -> Result<(), String> {
    match dest.as_mut() {
        Some(dest) => {
            let mut bytes = readme.into_bytes();
            // Append new line at end of file to match behavior of `cargo readme > README.md`
            bytes.push(b'\n');

            dest.write_all(&mut bytes).map(|_| ()).map_err(|e| {
                format!("Could not write to output file: {}", e)
            })?;
        }
        None => println!("{}", readme),
    }

    Ok(())
}

/// Find the default entrypoiny to read the doc comments from
///
/// Try to read entrypoint in the following order:
/// - src/main.rs
/// - src/lib.rs
/// - file defined in the `[lib]` section of Cargo.toml
/// - file defined in the `[[bin]]` section of Cargo.toml, if there is only one
///   - if there is more than one `[[bin]]`, an error is returned
pub fn find_entrypoint(current_dir: &Path) -> Result<File, String> {
    let lib_rs = current_dir.join("src/lib.rs");
    let main_rs = current_dir.join("src/main.rs");

    let cargo = try!(cargo_info::get_cargo_info(current_dir));

    // try src/main.rs
    match File::open(&main_rs) {
        Ok(file) => return Ok(file),
        Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
            return Err(format!(
                "Could not open file '{}': {}",
                main_rs.to_string_lossy(),
                e
            ))
        }
        _ => {}
    }

    // try src/lib.rs
    match File::open(&lib_rs) {
        Ok(file) => return Ok(file),
        Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
            return Err(format!(
                "Could not open file '{}': {}",
                lib_rs.to_string_lossy(),
                e
            ))
        }
        _ => {}
    }

    // try lib defined in `Cargo.toml`
    if let Some(lib) = cargo.lib {
        if let Some(ref path) = lib.path {
            match File::open(current_dir.join(&path)) {
                Ok(file) => return Ok(file),
                Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open file '{}': {}",
                        current_dir.join(&path).to_string_lossy(),
                        e
                    ));
                }
                _ => {}
            }
        }
    }

    // try bin defined in `Cargo.toml`
    match cargo.bin {
        // if there is only one, use it
        Some(ref bin_list) if bin_list.len() == 1 && bin_list[0].path.is_some() => {
            let path = &bin_list[0].path.clone().unwrap();
            match File::open(current_dir.join(path)) {
                Ok(file) => return Ok(file),
                Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open file '{}': {}",
                        current_dir.join(path).to_string_lossy(),
                        e
                    ))
                }
                _ => {}
            }
        }
        // if there is more than one, return an error
        Some(ref bin_list) if bin_list.len() > 1 => {
            let paths = bin_list
                .iter()
                .flat_map(|ref bin| bin.path.clone())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!("Multiple binaries found, choose one: [{}]", paths));
        }
        _ => {}
    }

    // if no entrypoint is found, return an error
    Err("No entrypoint found".to_owned())
}
