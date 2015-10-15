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
                .takes_value(true)
                .help("File to read from. If not provided, will take 'src/lib.rs' or 'src/main.rs'."))
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("File to write to. If not provided, will output to the console."))
            .arg(Arg::with_name("TEMPLATE")
                .short("t")
                .long("template")
                .takes_value(true)
                .help("Template used to render the output. Defaults to 'README.tpl'. \
                       If the default template is not found, \
                       the processed docstring will be used."))
            .arg(Arg::with_name("NO_INDENT_HEADINGS")
                .long("no-indent-headings")
                .help("Do not add an extra level to headings. \
                       By default, '#' headings become '##', \
                       so the first '#' can be your crate name. \
                       Use this option to prevent this behavior.\n"))
            .after_help("Input and output are relative to the current dir\n\n"))
        .get_matches();

    if let Some(m) = matches.subcommand_matches("readme") {
        let input = m.value_of("INPUT");
        let output = m.value_of("OUTPUT");
        let template = m.value_of("TEMPLATE");
        let indent_headings = !m.is_present("NO_INDENT_HEADINGS");

        process::execute(input, output, template, indent_headings);
    }
}
