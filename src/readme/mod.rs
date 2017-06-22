use std::io::Read;
use std::path::Path;

mod load;
mod extract;
mod transform;
mod template;

use self::extract::DocExtract;
use self::transform::DocTransform;
use cargo_info;

#[derive(PartialEq)]
enum DocStyle {
    NoDoc,
    SingleLine,
    MultiLine,
}

/// Generates readme data from `source` file
///
/// Optionally, a template can be used to render the output
pub fn generate_readme<T: Read>(
    project_root: &Path,
    source: &mut T,
    template: Option<&mut T>,
    add_title: bool,
    add_license: bool,
    indent_headings: bool,
) -> Result<String, String> {

    let readme = load::load_docs(source)
        .map_err(|e| format!("{}", e))?
        .extract_doc()
        .transform_doc(indent_headings)
        .fold(String::new(), |mut acc, x| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&x);
            acc
        });

    // get template from file
    let template = if let Some(template) = template {
        Some(get_template_string(template)?)
    } else {
        None
    };

    // get cargo info from Cargo.toml
    let cargo = cargo_info::get_cargo_info(project_root)?;
    if add_license && cargo.package.license.is_none() {
        return Err("License not found in Cargo.toml".to_owned());
    }

    template::render(template, readme, cargo, add_title, add_license)
}

/// Load a template String from a file
fn get_template_string<T: Read>(template: &mut T) -> Result<String, String> {
    let mut template_string = String::new();
    match template.read_to_string(&mut template_string) {
        Err(e) => return Err(format!("Error: {}", e)),
        _ => {}
    }

    Ok(template_string)
}
