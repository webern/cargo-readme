//! Generate README.md from docstrings
//!
//! Document your crate using docstrings to ensure your examples are correct and then generate the
//! `README.md` knowing the examples are still correct.
//!
//! # Usage
//! ```sh
//! cargo readme
//! ```

#[macro_use]
extern crate clap;
extern crate toml;

mod doc;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("cargo-readme")
        .version(&*format!("v{}", crate_version!()))
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo but we want usage strings to generated properly
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion,
                    AppSettings::SubcommandRequired])
        // We use a subcommand because everything parsed after `cargo` is sent to the third party
        // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("readme")
            .author("Livio Ribeiro <livioribeiro@outlook.com>")
            .about("Generate README.md from doc string")
            .arg(Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .help("File to read from. If not provided, will take 'src/lib.rs' or 'src/main.rs'.")
                .takes_value(true))
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("File to write to. If not provided, will output to the console.")
                .takes_value(true))
            .arg(Arg::with_name("NO_CRATE_NAME")
                .long("no-crate-name")
                .help("Do not prepend crate name to the output"))
            .arg(Arg::with_name("NO_INDENT_HEADINGS")
                .long("no-indent-headings")
                .help("Do not add an extra level to headings. \
                       By default, '#' headings become '##', \
                       so the first '#' can be your crate name. \
                       Use this option to prevent this behavior.\n"))
            .after_help("Input and output are relative to the current dir\n"))
        .get_matches();

    if let Some(m) = matches.subcommand_matches("readme") {
        match execute(m) {
            Err(e) => println!("{}", e),
            _ => {}
        }
    }
}

fn execute(matches: &ArgMatches) -> io::Result<()> {
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
        data.insert(0, format!("# {}", crate_name));
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
