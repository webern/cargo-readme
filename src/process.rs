use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use toml;

use regex::Regex;
use ::doc;

const DEFAULT_TEMPLATE: &'static str = "README.tpl";

fn get_crate_name() -> Result<String, String> {
    let current_dir = env::current_dir().unwrap();

    let mut cargo_toml = match File::open(current_dir.join("Cargo.toml")) {
        Ok(file) => file,
        Err(_) => return Err("Missing 'Cargo.toml'".to_owned()),
    };

    let mut buf = String::new();
    match cargo_toml.read_to_string(&mut buf) {
        Err(e) => return Err(format!("Error: {}", e)),
        Ok(_) => {},
    }

    let table = toml::Parser::new(&buf).parse().unwrap();
    let crate_name = table["package"].lookup("name").unwrap().as_str().unwrap();

    Ok(crate_name.to_owned())
}

fn fix_headings(data: Vec<String>) -> Vec<String> {
    data.into_iter().map(|mut line| {
        if line.starts_with("#") {
            line.insert(0, '#');
        }
        line
    }).collect()
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
            data[0].to_owned(), |acc, line| format!("{}\n{}", acc, line))
    }
}

fn get_template(template: Option<&str>) -> Result<Option<String>, String> {
    let current_dir = env::current_dir().unwrap();
    let template_name = template.unwrap_or(DEFAULT_TEMPLATE);

    let mut template_file = match File::open(current_dir.join(template_name)) {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("Error: {}", e)),
    };

    let mut template = String::new();
    match template_file.read_to_string(&mut template) {
        Err(e) => return Err(format!("Error: {}", e)),
        _ => {}
    }

    Ok(Some(template))
}

fn process_template(template: Option<&str>, data: Vec<String>) -> Result<String, String> {
    let crate_name = try!(get_crate_name());
    let docs = fold_data(data);

    match get_template(template) {
        Ok(Some(tpl)) => {
            let re_crate = Regex::new(r"\{\{crate\}\}").unwrap();
            let mut result = re_crate.replace(&tpl, &*crate_name);

            let re_docs = Regex::new(r"\{\{docs\}\}").unwrap();
            result = re_docs.replace(&result, &*docs);

            Ok(result)
        },
        Ok(None) => Ok(docs),
        Err(e) => Err(e),
    }
}

pub fn execute(input: Option<&str>, output: Option<&str>, template: Option<&str>, indent_headings: bool) {
    let current_dir = env::current_dir().unwrap();

    let mut source: File;

    if let Some(input) = input {
        let input = current_dir.join(input);
        source = match File::open(&input) {
            Ok(file) => file,
            Err(_) => {
                println!("File not found at {}", input.to_string_lossy());
                return;
            },
        };
    }
    else {
        let lib_rs = current_dir.join("src/lib.rs");
        let main_rs = current_dir.join("src/main.rs");
        source = match File::open(lib_rs).or_else(|_| File::open(main_rs)) {
            Ok(file) => file,
            Err(_) => {
                println!("No 'lib.rs' nor 'main.rs' were found");
                return;
            },
        };
    }

    let mut data: Vec<_> = doc::read(&mut source);

    if indent_headings {
        data = fix_headings(data);
    }

    let docs = match process_template(template, data) {
        Ok(docs) => docs,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if let Some(output) = output {
        let result = File::create(current_dir.join(output))
            .and_then(|mut f| f.write_all(docs.as_bytes()));

        if let Err(e) = result {
            println!("Error: {}", e);
        }
    }
    else {
        println!("{}", docs);
    }
}
