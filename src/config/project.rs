use std::env;
use std::path::{Path, PathBuf};

use crate::config::manifest::{Manifest, ManifestLib};

/// Get the project root from given path or defaults to current directory
///
/// The given path is appended to the current directory if is a relative path, otherwise it is used
/// as is. If no path is given, the current directory is used.
/// A `Cargo.toml` file must be present is the root directory.
pub fn get_root(given_root: Option<&str>) -> Result<PathBuf, String> {
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

    resolve_virtual_manifest(root)
}

/// Point a virtual manifest at the package it unambiguously stands for
///
/// A workspace root without a `[package]` section documents nothing itself, so a README has to be
/// generated for one of its members. Only a workspace naming exactly one package resolves without
/// asking; anything else is a choice the caller has to make with `--project-root`.
fn resolve_virtual_manifest(root: PathBuf) -> Result<PathBuf, String> {
    let manifest = cargo_toml::Manifest::<toml::Value>::from_path(root.join("Cargo.toml"))
        .map_err(|e| format!("Could not read Cargo.toml: {}", e))?;

    if manifest.package.is_some() {
        return Ok(root);
    }

    // Not a workspace either: leave the manifest to report its own missing `[package]`.
    let Some(workspace) = manifest.workspace else {
        return Ok(root);
    };

    let patterns = if workspace.default_members.is_empty() {
        &workspace.members
    } else {
        &workspace.default_members
    };

    let mut members = Vec::new();
    for pattern in patterns {
        for member in expand_member(&root, pattern)? {
            if !workspace.exclude.contains(&member) && !members.contains(&member) {
                members.push(member);
            }
        }
    }
    members.sort();

    match members.len() {
        1 => Ok(root.join(&members[0])),
        0 => Err("Cargo.toml is a virtual manifest with no workspace members".to_owned()),
        _ => Err(format!(
            "Multiple workspace members found, choose one with --project-root: [{}]",
            members.join(", ")
        )),
    }
}

/// Expand one `[workspace] members` entry, which may be a glob, into the packages it names
fn expand_member(root: &Path, pattern: &str) -> Result<Vec<String>, String> {
    if !pattern.contains(['*', '?', '[']) {
        return Ok(vec![pattern.to_owned()]);
    }

    let absolute = root.join(pattern);
    let paths = glob::glob(&absolute.to_string_lossy())
        .map_err(|e| format!("Invalid workspace member pattern '{}': {}", pattern, e))?;

    Ok(paths
        .filter_map(Result::ok)
        .filter(|path| path.join("Cargo.toml").is_file())
        .filter_map(|path| {
            path.strip_prefix(root)
                .ok()
                .map(|rel| rel.to_string_lossy().replace('\\', "/"))
        })
        .collect())
}

/// Find the default entrypoiny to read the doc comments from
///
/// Try to read entrypoint in the following order:
/// - src/lib.rs
/// - src/main.rs
/// - file defined in the `[lib]` section of Cargo.toml
/// - file defined in the `[[bin]]` section of Cargo.toml, if there is only one
///   - if there is more than one `[[bin]]`, an error is returned
pub fn find_entrypoint(current_dir: &Path, manifest: &Manifest) -> Result<PathBuf, String> {
    // try lib.rs
    let lib_rs = current_dir.join("src/lib.rs");
    if lib_rs.exists() {
        return Ok(lib_rs);
    }

    // try main.rs
    let main_rs = current_dir.join("src/main.rs");
    if main_rs.exists() {
        return Ok(main_rs);
    }

    // try lib defined in `Cargo.toml`
    if let Some(ManifestLib {
        path: ref lib,
        doc: true,
    }) = manifest.lib
    {
        return Ok(lib.to_path_buf());
    }

    // try bin defined in `Cargo.toml`
    if !manifest.bin.is_empty() {
        let mut bin_list: Vec<_> = manifest
            .bin
            .iter()
            .filter(|b| b.doc)
            .map(|b| b.path.clone())
            .collect();

        if bin_list.len() > 1 {
            let paths = bin_list
                .iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!("Multiple binaries found, choose one: [{}]", paths));
        }

        if let Some(bin) = bin_list.pop() {
            return Ok(bin);
        }
    }

    // if no entrypoint is found, return an error
    Err("No entrypoint found".to_owned())
}
