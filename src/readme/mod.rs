use std::io::{BufRead, BufReader, Read};
use std::path::Path;

mod extract;
mod process;
mod template;

use crate::config;

/// Toggles controlling how the README is generated.
///
/// Bundling these flags keeps `generate_readme` free of a long run of same-typed `bool`
/// arguments, which are easy to transpose at the call site.
#[derive(Debug, Clone, Copy)]
pub struct ReadmeOptions {
    /// Prepend the crate name as a title. Ignored when using a template.
    pub add_title: bool,
    /// Prepend the badges defined in `Cargo.toml`. Ignored when using a template.
    pub add_badges: bool,
    /// Append the license defined in `Cargo.toml`. Ignored when using a template.
    pub add_license: bool,
    /// Add an extra level to headings so the crate name can be the top heading.
    pub indent_headings: bool,
    /// Extract docs from comments rather than processing the input file as-is.
    pub extract_from_comment: bool,
}

impl Default for ReadmeOptions {
    fn default() -> Self {
        ReadmeOptions {
            add_title: true,
            add_badges: true,
            add_license: true,
            indent_headings: true,
            extract_from_comment: true,
        }
    }
}

/// Generates readme data from `source` file
///
/// Optionally, a template can be used to render the output
pub fn generate_readme<T: Read>(
    project_root: &Path,
    source: &mut T,
    template: Option<&mut T>,
    options: ReadmeOptions,
) -> Result<String, String> {
    let lines = if options.extract_from_comment {
        extract::extract_docs(source).map_err(|e| format!("{}", e))?
    } else {
        BufReader::new(source)
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{}", e))?
    };

    let readme = process::process_docs(lines, options.indent_headings).join("\n");

    // get template from file
    let template = if let Some(template) = template {
        Some(get_template_string(template)?)
    } else {
        None
    };

    // get manifest from Cargo.toml
    let cargo = config::get_manifest(project_root)?;

    template::render(template, readme, &cargo, options)
}

/// Load a template String from a file
fn get_template_string<T: Read>(template: &mut T) -> Result<String, String> {
    let mut template_string = String::new();
    if let Err(e) = template.read_to_string(&mut template_string) {
        return Err(format!("Error: {}", e));
    }

    Ok(template_string)
}
