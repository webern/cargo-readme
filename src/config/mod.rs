use std::path::{Path, PathBuf};

mod manifest;
mod project;

use self::manifest::Manifest;

pub struct ReadmeConfig {
    pub root: PathBuf,
    pub input: PathBuf,
    pub template: Option<PathBuf>,
    pub add_title: bool,
    pub add_badges: bool,
    pub add_license: bool,
    pub indent_headings: bool,
}

pub struct ReadmeConfigDefaults<'a> {
    pub root: Option<&'a Path>,
    pub input: Option<&'a Path>,
    pub template: Option<&'a Path>,
    pub add_title: Option<bool>,
    pub add_badges: Option<bool>,
    pub add_license: Option<bool>,
    pub indent_headings: Option<bool>
}

impl<'a> Default for ReadmeConfigDefaults<'a> {
    fn default() -> Self {
        ReadmeConfigDefaults {
            root: None,
            input: None,
            template: None,
            add_title: None,
            add_badges: None,
            add_license: None,
            indent_headings: None,
        }
    }
}

pub fn get_config(defaults: ReadmeConfigDefaults, manifest: &Manifest) -> Result<ReadmeConfig, String> {
    let root = defaults.root
        .map(|root| root.to_path_buf())
        .unwrap_or(project::get_root(None)?);

    let input = defaults.input
        .map(|ep| ep.to_path_buf())
        .unwrap_or(project::find_input(root.as_ref(), manifest)?);

    let template = defaults.template
        .map(|tpl| tpl.to_path_buf());

    let add_title = defaults.add_title.unwrap_or(true);
    let add_badges = defaults.add_badges.unwrap_or(true);
    let add_license = defaults.add_license.unwrap_or(true);
    let indent_headings = defaults.indent_headings.unwrap_or(true);

    Ok(ReadmeConfig {
        root: root,
        input: input,
        template: template,
        add_title: add_title,
        add_badges: add_badges,
        add_license: add_license,
        indent_headings: indent_headings
    })
}
