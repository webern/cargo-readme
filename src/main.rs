//! Generate README.md from doc comments.

#[macro_use] extern crate clap;

extern crate cargo_readme;

use std::io::{self, Write};

use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

mod helper;

fn main() {
    let matches = App::new("cargo-readme")
        .version(&*format!("v{}", crate_version!()))
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo but we want usage strings to generated properly
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion, AppSettings::SubcommandRequired])
        // We use a subcommand because everything parsed after `cargo` is sent to the third party
        // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("readme")
            .author("Livio Ribeiro <livioribeiro@outlook.com>")
            .about("Generate README.md from doc comments")
            .arg(Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("File to read from.{n}\
                       If not provided, will try to use `src/main.rs`, then `src/lib.rs`. If \
                       neither file could be found, will look into `Cargo.toml` for a `[lib]`, \
                       then for a single `[[bin]]`. If multiple binaries are found, you will be \
                       asked to choose one."))
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("File to write to. If not provided, will output to stdout."))
            .arg(Arg::with_name("ROOT")
                .short("r")
                .long("project-root")
                .takes_value(true)
                .help("Directory to be set as project root (where `Cargo.toml` is){n}\
                       Defaults to the current directory."))
            .arg(Arg::with_name("TEMPLATE")
                .short("t")
                .long("template")
                .takes_value(true)
                .conflicts_with("NO_TEMPLATE")
                .help("Template used to render the output.{n}\
                       Default behavior is to use `README.tpl` if it exists."))
            .arg(Arg::with_name("NO_TITLE")
                .long("no-title")
                .help("Do not prepend title line.{n}\
                       By default, the title ('# crate-name') is prepended to the output.{n}\
                       Ignored when using a template."))
            .arg(Arg::with_name("NO_BADGES")
                .long("no-badges")
                .help("Do not prepend badges line.{n}\
                       By default, badges defined in Cargo.toml are prepended to the output.{n}\
                       Ignored when using a template."))
            .arg(Arg::with_name("NO_LICENSE")
                .long("no-license")
                .help("Do not append license line.{n}\
                       By default, the license defined in `Cargo.toml` will be prepended to the output.{n}\
                       Ignored when using a template."))
            .arg(Arg::with_name("NO_TEMPLATE")
                .long("no-template")
                .help("Ignore template file when generating README.{n}\
                       Only useful to ignore default template `README.tpl`."))
            .arg(Arg::with_name("NO_INDENT_HEADINGS")
                .long("no-indent-headings")
                .help("Do not add an extra level to headings.{n}\
                       By default, '#' headings become '##', so the first '#' can be the crate \
                       name. Use this option to prevent this behavior.{n}")))
        .get_matches();

    if let Some(m) = matches.subcommand_matches("readme") {
        match execute(m) {
            Err(e) => {
                io::stderr()
                    .write_fmt(format_args!("Error: {}\n", e))
                    .expect("An error occurred while trying to show an error message");
                std::process::exit(1);
            }
            _ => {}
        }
    }
}

/// Takes the arguments matches from clap and outputs the result, either to stdout of a file
fn execute(m: &ArgMatches) -> Result<(), String> {
    // get inputs
    let input = m.value_of("INPUT");
    let output = m.value_of("OUTPUT");
    let template = m.value_of("TEMPLATE");
    let add_title = !m.is_present("NO_TITLE");
    let add_badges = !m.is_present("NO_BADGES");
    let add_license = !m.is_present("NO_LICENSE");
    let no_template = m.is_present("NO_TEMPLATE");
    let indent_headings = !m.is_present("NO_INDENT_HEADINGS");

    // get project root
    let project_root = helper::get_project_root(m.value_of("ROOT"))?;

    // get source file
    let mut source = helper::get_source(&project_root, input)?;

    // get destination file
    let mut dest = helper::get_dest(&project_root, output)?;

    // get template file
    let mut template_file = if no_template {
        None
    } else {
        helper::get_template_file(&project_root, template)?
    };

    // generate output
    let readme = cargo_readme::generate_readme(
        &project_root,
        &mut source,
        template_file.as_mut(),
        add_title,
        add_badges,
        add_license,
        indent_headings,
    )?;

    helper::write_output(&mut dest, readme)
}
