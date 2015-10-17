use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use toml;

const SRC_RUST: &'static str = "SRC_RUST";
const SRC_OTHER: &'static str = "SRC_OTHER";
const SRC_DOC: &'static str = "SRC_DOC";

pub fn generate_readme<T: Read>(source: &mut T, template: &mut Option<T>, indent_headings: bool) -> Result<String, String> {
    let doc_data = extract(source, indent_headings);

    match template.as_mut() {
        Some(template) => process_template(template, doc_data),
        None => Ok(fold_data(doc_data)),
    }
}

pub fn extract<T: Read>(source: &mut T, indent_headings: bool) -> Vec<String> {
    let reader = BufReader::new(source);

    let re_code_rust = Regex::new(r"^//! ```(no_run|ignore|should_panic)?$").unwrap();
    let re_code_other = Regex::new(r"//! ```\w+").unwrap();

    let mut section = SRC_DOC;

    reader.lines().filter_map(|line| {
        let mut line = line.unwrap();
        if line.starts_with("//!") {

            if  section == SRC_DOC && re_code_rust.is_match(&line) {
                section = SRC_RUST;

                return Some("```rust".to_owned());
            }
            else if section == SRC_DOC && re_code_other.is_match(&line) {
                section = SRC_OTHER;
            }
            else if section != SRC_DOC && line == "//! ```" {
                section = SRC_DOC;

                return Some("```".to_owned());
            }

            // If line is hidden in documentation, it is also hidden in README
            if section == SRC_RUST && line.starts_with("//! # ") {
                return None;
            }

            // Remove leading '//!' before returning the line
            if line.trim() == "//!" {
                line = String::new();
            }
            else {
                line = line[4..].to_owned();
                // If we should indent headings, only do this outside code blocks
                if indent_headings && section == SRC_DOC && line.starts_with("#") {
                    line.insert(0, '#');
                }
            }
            
            Some(line)
        }
         else {
            return None
        }
    })
    .collect()
}

pub fn process_template<T: Read>(template: &mut T, data: Vec<String>) -> Result<String, String> {
    let crate_name = try!(get_crate_name());
    let docs = fold_data(data);

    let template = try!(get_template(template));

    let mut result;
    result = Regex::new(r"\{\{crate\}\}").unwrap().replace(&template, &*crate_name);
    result = Regex::new(r"\{\{docs\}\}").unwrap().replace(&result, &*docs);

    Ok(result)
}

fn get_crate_name() -> Result<String, String> {
    let current_dir = env::current_dir().unwrap();

    let mut cargo_toml = match File::open(current_dir.join("Cargo.toml")) {
        Ok(file) => file,
        Err(_) => return Err(format!("'Cargo.toml' not found in '{}'", current_dir.to_string_lossy())),
    };

    let mut buf = String::new();
    match cargo_toml.read_to_string(&mut buf) {
        Err(e) => return Err(format!("{}", e)),
        Ok(_) => {},
    }

    let table = toml::Parser::new(&buf).parse().unwrap();
    let crate_name = table["package"].lookup("name").unwrap().as_str().unwrap();

    Ok(crate_name.to_owned())
}

fn fold_data(data: Vec<String>) -> String {
    if data.len() < 1 {
        String::new()
    }
    else if data.len() < 2 {
        data[0].to_owned()
    }
    else {
        data[1..].into_iter().fold(
            data[0].to_owned(), |acc, line| format!("{}\n{}", acc, line)
        )
    }
}

fn get_template<T: Read>(template: &mut T) -> Result<String, String> {
    let mut template_string = String::new();
    match template.read_to_string(&mut template_string) {
        Err(e) => return Err(format!("Error: {}", e)),
        _ => {}
    }

    Ok(template_string)
}
