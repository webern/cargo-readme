use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use toml;

const SRC_RUST: &'static str = "SRC_RUST";
const SRC_OTHER: &'static str = "SRC_OTHER";
const SRC_DOC: &'static str = "SRC_DOC";

pub fn extract<T: Read>(source: &mut T) -> Vec<String> {
    let reader = BufReader::new(source);

    let re_code_rust = Regex::new(r"^//! ```(no_run|ignore|should_panic)?$").unwrap();
    let re_code_other = Regex::new(r"//! ```\w+").unwrap();

    let mut section = SRC_DOC;

    reader.lines().filter_map(|line| {
        let line = line.unwrap();
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

            if section == SRC_RUST && line.starts_with("//! # ") {
                return None;
            }

            // Remove leading '//!' before returning the line
            if line.len() == 3 {
                Some("".to_owned())
            }
            else {
                Some(line[4..].to_owned())
            }
        }
         else {
            return None
        }
    })
    .collect()
}

pub fn process<T: Read>(mut data: Vec<String>, template: &mut Option<T>, indent_headings: bool) -> Result<String, String> {
    if indent_headings {
        data = data.into_iter().map(|mut line| {
            if line.starts_with("#") {
                line.insert(0, '#');
            }
            line
        }).collect();
    }

    let docs = match process_template(template, data) {
        Ok(docs) => docs,
        Err(e) => return Err(format!("{}", e)),
    };

    Ok(docs)
}

fn process_template<T: Read>(template: &mut Option<T>, data: Vec<String>) -> Result<String, String> {
    let crate_name = try!(get_crate_name());
    let docs = fold_data(data);

    match template.as_mut() {
        Some(tpl) => {
            let tpl = try!(get_template(tpl));

            let mut result;
            result = Regex::new(r"\{\{crate\}\}").unwrap().replace(&tpl, &*crate_name);
            result = Regex::new(r"\{\{docs\}\}").unwrap().replace(&result, &*docs);

            Ok(result)
        },
        None => Ok(docs),
    }
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
