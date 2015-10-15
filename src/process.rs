use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use clap::ArgMatches;
use toml;

use ::doc;

pub fn execute(matches: &ArgMatches) -> io::Result<()> {
    let current_dir = env::current_dir().unwrap();

    let mut source: File;

    if let Some(input) = matches.value_of("INPUT") {
        source = try!(File::open(current_dir.join(input)));
    }
    else {
        source = try!(File::open(current_dir.join("src/lib.rs"))
            .or_else(|_| File::open(current_dir.join("src/main.rs"))));
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
        let mut cargo_toml = File::open(current_dir.join("Cargo.toml")).unwrap();
        let mut buf = String::new();
        cargo_toml.read_to_string(&mut buf).unwrap();

        let table = toml::Parser::new(&buf).parse().unwrap();
        let crate_name = table["package"].lookup("name").unwrap().as_str().unwrap();
        data.insert(0, format!("# {}\n", crate_name));
    }

    if let Some(output) = matches.value_of("OUTPUT") {
        let mut dest = try!(File::create(current_dir.join(output)));
        try!(doc::write(&mut dest, &data));
    }
    else {
        for line in data {
            println!("{}", line);
        }
    }

    Ok(())
}
