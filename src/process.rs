use std::env;
use std::fs::File;
use std::io::prelude::*;
use clap::ArgMatches;
use toml;

use ::doc;

pub fn execute(matches: &ArgMatches) -> Result<(), String> {
    let current_dir = env::current_dir().unwrap();

    let mut source: File;

    if let Some(input) = matches.value_of("INPUT") {
        let input = current_dir.join(input);
        source = match File::open(&input) {
            Ok(file) => file,
            Err(_) => return Err(format!("File not found at {}", input.to_string_lossy())),
        };
    }
    else {
        let lib_rs = current_dir.join("src/lib.rs");
        let main_rs = current_dir.join("src/main.rs");
        source = match File::open(lib_rs).or_else(|_| File::open(main_rs)) {
            Ok(file) => file,
            Err(_) => return Err("No 'lib.rs' nor 'main.rs' were found".to_owned()),
        };
    }

    let mut data: Vec<_> = doc::read(&mut source);

    if !matches.is_present("NO_INDENT_HEADINGS") {
        data = data.into_iter().map(|mut line| {
            if line.starts_with("#") {
                line.insert(0, '#');
            }
            line
        }).collect();
    }

    if !matches.is_present("NO_CRATE_NAME") {
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
        data.insert(0, format!("# {}\n", crate_name));
    }

    if let Some(output) = matches.value_of("OUTPUT") {
        let mut dest = match File::create(current_dir.join(output)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error: {}", e)),
        };
        match doc::write(&mut dest, &data) {
            Err(e) => return Err(format!("Error: {}", e)),
            Ok(_) => {},
        }
    }
    else {
        for line in data {
            println!("{}", line);
        }
    }

    Ok(())
}
