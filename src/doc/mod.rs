use std::io::Read;
use std::path::Path;

mod extractor;
mod modifier;
mod cargo_info;
mod template;

use self::extractor::DocExtractor;
use self::modifier::DocModify;
pub use self::cargo_info::{Cargo, get_cargo_info};

/// Generates readme data from `source` file
pub fn generate_readme<T: Read>(project_root: &Path,
                                source: &mut T,
                                template: Option<&mut T>,
                                add_title: bool,
                                add_license: bool,
                                indent_headings: bool)
    -> Result<String, String> {

    // ceate doc extractor
    let doc_iter = DocExtractor::new(source)
        .modify_doc(indent_headings);

    let mut doc_data = Vec::new();
    for line in doc_iter {
        let line = line.map_err(|e| format!("{}", e))?;
        doc_data.push(line);
    }
    
    // fold doc_data (Vec<String>) into single String
    let readme = fold_doc_data(doc_data);

    // get template from file
    let template = if let Some(template) = template {
        Some(get_template(template)?)
    } else {
        None
    };

    // get cargo info from Cargo.toml
    let cargo = cargo_info::get_cargo_info(project_root)?;
    if add_license && cargo.package.license.is_none() {
        return Err("License not found in Cargo.toml".to_owned());
    }

    render(template, readme, cargo, add_title, add_license)
}

/// Transforms the Vec of lines into a single String
fn fold_doc_data(data: Vec<String>) -> String {
    if data.len() < 1 {
        String::new()
    } else if data.len() < 2 {
        data[0].to_owned()
    } else {
        data[1..].into_iter().fold(data[0].to_owned(), |acc, line| format!("{}\n{}", acc, line))
    }
}

/// Load a template String from a file
fn get_template<T: Read>(template: &mut T) -> Result<String, String> {
    let mut template_string = String::new();
    match template.read_to_string(&mut template_string) {
        Err(e) => return Err(format!("Error: {}", e)),
        _ => {}
    }

    Ok(template_string)
}

fn render(template: Option<String>, mut readme: String, cargo: Cargo, add_title: bool, add_license: bool) -> Result<String, String> {
    let title = cargo.package.name.as_ref();
    let license = cargo.package.license.as_ref();

    match template {
        Some(template) => {
            if template.contains("{{license}}") && !add_license {
                return Err("`{{license}}` was found in template but should not be rendered".to_owned());
            }

            if template.contains("{{crate}}") && !add_title {
                return Err("`{{crate}}` was found in template but title should not be rendered".to_owned());
            }

            let title = if add_title { Some(title) } else { None };
            let license = if add_license { Some(license.unwrap().as_ref()) } else { None };
            template::process_template(template, readme, title, license)
        }
        None => {
            if add_title {
                readme = template::prepend_title(readme, &title);
            }
            if add_license {
                readme = template::append_license(readme, &license.unwrap());
            }

            Ok(readme)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fold_data_empty_input() {
        let input: Vec<String> = vec![];

        let result = super::fold_doc_data(input);

        assert!(result.is_empty());
    }

    #[test]
    fn fold_data_single_line() {
        let line = "# single line";
        let input: Vec<String> = vec![line.to_owned()];

        let result = super::fold_doc_data(input);

        assert_eq!(line, result);
    }

    #[test]
    fn fold_data_multiple_lines() {
        let input: Vec<String> = vec![
            "# first line".to_owned(),
            "second line".to_owned(),
            "third line".to_owned(),
        ];

        let result = super::fold_doc_data(input);

        assert_eq!("# first line\nsecond line\nthird line", result);
    }
}