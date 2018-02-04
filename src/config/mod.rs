mod manifest;
mod project;

use std::path::{Path, PathBuf};

pub struct ReadmeConfig {
    pub project_root: PathBuf,
    pub entrypoint: PathBuf,
    pub template: Option<PathBuf>,
    pub add_title: bool,
    pub add_badges: bool,
    pub add_license: bool,
    pub indent_headings: bool,
}

pub struct ReadmeConfigDefaults<'a> {
    pub project_root: Option<&'a Path>,
    pub entrypoint: Option<&'a Path>,
    pub template: Option<&'a Path>,
    pub add_title: Option<bool>,
    pub add_badges: Option<bool>,
    pub add_license: Option<bool>,
    pub indent_headings: Option<bool>
}

impl<'a> Default for ReadmeConfigDefaults<'a> {
    fn default() -> Self {
        ReadmeConfigDefaults {
            project_root: None,
            entrypoint: None,
            template: None,
            add_title: None,
            add_badges: None,
            add_license: None,
            indent_headings: None,
        }
    }
}

pub fn get_config(defaults: ReadmeConfigDefaults) -> Result<ReadmeConfig, String> {
    let project_root = defaults.project_root
        .map(|root| root.to_path_buf())
        .ok_or_else(project::get_project_root(None))?;

    let entrypoint = defaults.entrypoint
        .map(|ep| ep.to_path_buf())
        .ok_or_else(project::find_entrypoint(current_dir))

    Err(String::new())
}
