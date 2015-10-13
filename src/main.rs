#[macro_use]
extern crate clap;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{App, AppSettings, SubCommand};

fn extract(source: &mut File) -> Vec<String> {
    let reader = BufReader::new(source);
    let mut in_code = false;

    reader.lines().filter_map(|line| {
        let line = line.unwrap();
        if line.starts_with("//!") {
            if line.starts_with("//! ```") {
                in_code = !in_code;
            }
            else if line.starts_with("//! # ") && in_code {
                return None;
            }

            // Remove leading '//!' before returning the line
            if line.len() == 3 {
                Some("".to_owned())
            }
            else {
                Some(line[4..].to_owned())
            }
        } else {
            return None
        }
    })
    .collect()
}

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
            .about("Generate README.md from doc string"))
        .get_matches();

    if matches.subcommand_matches("readme").is_some() {
        let current_dir = env::current_dir().unwrap();
        let mut source = File::open(current_dir.join("src/lib.rs"))
            .unwrap_or_else(|_| File::open(current_dir.join("src/main.rs"))
                .unwrap_or_else(|_| panic!("No 'lib.rs' nor 'main.rs' found")));

        let data: Vec<_> = extract(&mut source);

        println!("{}", data.iter().fold(String::new(), |acc, ref item| format!("{}{}\n", acc, item)));
    }
}
