//! Generate README.md from docstrings
//!
//! Document your crate using docstrings to ensure your examples are correct and then generate the
//! `README.md` knowing the examples are still correct.
//!
//! # Usage
//! ```sh
//! $ cargo readme [options]
//! ```

#[macro_use]
extern crate clap;
extern crate toml;
extern crate regex;

mod doc;
mod process;

use clap::{Arg, App, AppSettings, SubCommand};

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
        match process::execute(m) {
            Err(e) => println!("{}", e),
            _ => {}
        }
    }
}
