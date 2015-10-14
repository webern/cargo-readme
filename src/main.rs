#[macro_use]
extern crate clap;

mod doc;

use std::env;
use std::fs::File;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

fn execute(matches: &ArgMatches) {
    let current_dir = env::current_dir().unwrap();

    let mut source: File;

    if let Some(input) = matches.value_of("INPUT") {
        source = File::open(current_dir.join(input)).unwrap_or_else(|e| {
            let error = format!("{}", e);
            panic!(error);
        });
    } else {
        source = File::open(current_dir.join("src/lib.rs"))
            .unwrap_or_else(|_| File::open(current_dir.join("src/main.rs"))
                .unwrap_or_else(|_| panic!("No 'lib.rs' nor 'main.rs' found")));
    }

    let data: Vec<_> = doc::read(&mut source);

    if let Some(output) = matches.value_of("OUTPUT") {
        let mut dest = File::create(current_dir.join(output)).unwrap_or_else(|e| {
            let error = format!("{}", e);
            panic!(error);
        });
        doc::write(&mut dest, &data).unwrap();
    }
    else {
        for line in data {
            println!("{}", line);
        }
    }

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
            .after_help("Input and output are relative to the current dir"))
        .get_matches();

    if let Some(m) = matches.subcommand_matches("readme") {
        execute(m);
    }
}
