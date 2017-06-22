//! Generate README.md from doc comments.
//!
//! # Installation
//!
//! Just clone this repository, run `cargo build --release` and add `target/release/cargo-readme`
//! somewhere in your path.
//!
//! # About
//!
//! This cargo subcommand will extract documentation from your crate's doc comments
//! that you can use to populate its README.md.
//!
//! For example, if your crate has the following doc comments at `lib.rs`
//!
//! ```rust
//! //! This is my awesome crate
//! //!
//! //! Here goes some other description of what it is and what is does
//! //!
//! //! # Examples
//! //! ```
//! //! fn sum2(n1: i32, n2: i32) -> i32 {
//! //!   n1 + n2
//! //! }
//! //! # assert_eq!(4, sum2(2, 2));
//! //! ```
//! ```
//!
//! you may want to use it as your `README.md` content (without rust's doc comments specific stuff, obviously)
//! so you don't have to maintain the same documentation in the different places.
//!
//! Using `cargo-readme`, you write the documentation as doc comments, fill README.md with it and
//! you can be sure that the examples are correct.
//!
//! The result would look like:
//!
//!     # crate-name
//!
//!     This is my awesome crate
//!
//!     Here goes some other description of what it is and what is does
//!
//!     ## Examples
//!     ```rust
//!     fn sum2(n1: i32, n2: i32) -> i32 {
//!       n1 + n2
//!     }
//!     ```
//!
//! You may have noticed that `# Examples` became `## Examples`. This is intentional (and can be disabled)
//! so in README.md the first heading can be your crate name.
//!
//! Also, the crate name was automatically added (can be disabled too). It is read
//! from `Cargo.toml` so you just need to have them there. License can be read from
//! `Cargo.toml`, but it's opt-in.
//!
//! If you have additional information that does not fit in doc comments, you can use
//! a template. To do so, just create a file called `README.tpl` in the same directory
//! as `Cargo.toml` with the following content
//!
//!     Your crate's badges here
//!
//!     {{readme}}
//!
//!     Some additional info here
//!
//! The output will look like this
//!
//!     # crate-name
//!
//!     Your crate's badges here
//!
//!     This is my awesome crate
//!
//!     Here goes some other description of what it is and what is does
//!
//!     ## Examples
//!     ```rust
//!     fn sum2(n1: i32, n2: i32) -> i32 {
//!       n1 + n2
//!     }
//!     ```
//!
//!     Some additional info here
//!
//! You can override the displaying of your crate's name and license using `{{crate}}`
//! and `{{license}}`.

#[macro_use]
extern crate clap;

extern crate cargo_readme;

use std::env;
use std::io::{self, Write, ErrorKind};
use std::fs::File;
use std::path::{Path, PathBuf};
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

use cargo_readme::cargo_info;

const DEFAULT_TEMPLATE: &'static str = "README.tpl";

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
                .help("File to read from. If not provided, will take 'src/lib.rs' or 'src/main.rs'."))
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
                       Defaults to the current directory"))
            .arg(Arg::with_name("TEMPLATE")
                .short("t")
                .long("template")
                .takes_value(true)
                .conflicts_with("NO_TEMPLATE")
                .help("Template used to render the output. Defaults to 'README.tpl'.{n}\
                       If the default template is not found, the processed docstring\
                       will be used.{n}"))
            .arg(Arg::with_name("NO_TITLE")
                .long("no-title")
                .help("Do not prepend title line. By default, the title ('# crate-name') is{n}\
                       prepended to the output. If a template is used and it contains the tag{n}\
                       '{{crate}}', the template takes precedence and this option is ignored.{n}"))
            .arg(Arg::with_name("NO_LICENSE")
                .long("no-license")
                .help("Do not append license line. By default, the license, if defined in{n}\
                       `Cargo.toml`, will be prepended to the output. If a template is used{n}\
                       and it contains the tag '{{license}}', the template takes precedence and{n}\
                       this option is ignored.{n}"))
            .arg(Arg::with_name("NO_TEMPLATE")
                .long("no-template")
                .help("Ignore template file when generating README.{n}\
                       Only useful to ignore default template README.tpl.{n}"))
            .arg(Arg::with_name("NO_INDENT_HEADINGS")
                .long("no-indent-headings")
                .help("Do not add an extra level to headings.{n}\
                       By default, '#' headings become '##', so the first '#' can be your crate{n}\
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
    // get project root
    let project_root = get_project_root(m.value_of("ROOT"))?;

    // get inputs
    let input = m.value_of("INPUT");
    let output = m.value_of("OUTPUT");
    let template = m.value_of("TEMPLATE");
    let add_title = !m.is_present("NO_TITLE");
    let add_license = !m.is_present("NO_LICENSE");
    let no_template = m.is_present("NO_TEMPLATE");
    let indent_headings = !m.is_present("NO_INDENT_HEADINGS");

    // get source file
    let mut source = get_source(&project_root, input)?;

    // get destination file
    let mut dest = get_dest(&project_root, output)?;

    // get template file
    let mut template_file = if no_template {
        None
    } else {
        get_template_file(&project_root, template)?
    };

    // generate output
    let readme = cargo_readme::generate_readme(
        &project_root,
        &mut source,
        template_file.as_mut(),
        add_title,
        add_license,
        indent_headings,
    )?;

    write_output(&mut dest, readme)
}

/// Get the project root from given path or defaults to current directory
///
/// The given path is appended to the current directory if is a relative path, otherwise it is used
/// as is. If no path is given, the current directory is used.
/// A `Cargo.toml` file must be present is the root directory.
fn get_project_root(given_root: Option<&str>) -> Result<PathBuf, String> {
    let current_dir = env::current_dir().map_err(|e| format!("{}", e))?;
    let root = match given_root {
        Some(root) => {
            let root = Path::new(root);
            if root.is_absolute() {
                root.to_path_buf()
            } else {
                current_dir.join(root)
            }
        }
        None => current_dir,
    };

    if !root.join("Cargo.toml").is_file() {
        return Err(format!(
            "`{:?}` does not look like a Rust/Cargo project",
            root
        ));
    }

    Ok(root)
}

/// Get the source file from which the doc comments will be extracted
fn get_source(project_root: &Path, input: Option<&str>) -> Result<File, String> {
    match input {
        Some(input) => {
            let input = project_root.join(input);
            File::open(&input).map_err(|e| {
                format!("Could not open file '{}': {}", input.to_string_lossy(), e)
            })
        }
        None => find_entrypoint(&project_root),
    }
}

/// Get the destination file where the result will be output to
fn get_dest(project_root: &Path, output: Option<&str>) -> Result<Option<File>, String> {
    match output {
        Some(filename) => {
            let output = project_root.join(filename);
            File::create(&output).map(|f| Some(f)).map_err(|e| {
                format!(
                    "Could not create output file '{}': {}",
                    output.to_string_lossy(),
                    e
                )
            })
        }
        None => Ok(None),
    }
}

/// Get the template file that will be used to render the output
fn get_template_file(project_root: &Path, template: Option<&str>) -> Result<Option<File>, String> {
    match template {
        // template path was given, try to read it
        Some(template) => {
            let template = project_root.join(template);
            File::open(&template).map(|f| Some(f)).map_err(|e| {
                format!(
                    "Could not open template file '{}': {}",
                    template.to_string_lossy(),
                    e
                )
            })
        }
        // try to read the defautl template file
        None => {
            let template = project_root.join(DEFAULT_TEMPLATE);
            match File::open(&template) {
                Ok(file) => Ok(Some(file)),
                // do not generate an error on file not found
                Err(ref e) if e.kind() != ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open template file '{}': {}",
                        DEFAULT_TEMPLATE,
                        e
                    ))
                }
                // default template not found, return `None`
                _ => Ok(None),
            }
        }
    }
}

/// Write result to output, either stdout or destination file
fn write_output(dest: &mut Option<File>, readme: String) -> Result<(), String> {
    match dest.as_mut() {
        Some(dest) => {
            let mut bytes = readme.into_bytes();
            // Append new line at end of file to match behavior of `cargo readme > README.md`
            bytes.push(b'\n');

            dest.write_all(&mut bytes).map(|_| ()).map_err(|e| {
                format!("Could not write to output file: {}", e)
            })?;
        }
        None => println!("{}", readme),
    }

    Ok(())
}

/// Find the default entrypoiny to read the doc comments from
///
/// Try to read entrypoint in the following order:
/// - src/lib.rs
/// - src/main.rs
/// - file defined in the `[lib]` section of Cargo.toml
/// - file defined in the `[[bin]]` section of Cargo.toml, if there is only one
///   - if there is more than one `[[bin]]`, an error is returned
///
/// An error is returned if no entrypoint is found
fn find_entrypoint(current_dir: &Path) -> Result<File, String> {
    let lib_rs = current_dir.join("src/lib.rs");
    let main_rs = current_dir.join("src/main.rs");

    let cargo = try!(cargo_info::get_cargo_info(current_dir));

    // try src/lib.rs
    match File::open(&lib_rs) {
        Ok(file) => return Ok(file),
        Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
            return Err(format!(
                "Could not open file '{}': {}",
                lib_rs.to_string_lossy(),
                e
            ))
        }
        _ => {}
    }

    // try src/main.rs
    match File::open(&main_rs) {
        Ok(file) => return Ok(file),
        Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
            return Err(format!(
                "Could not open file '{}': {}",
                main_rs.to_string_lossy(),
                e
            ))
        }
        _ => {}
    }

    // try lib defined in `Cargo.toml`
    match cargo.lib {
        Some(lib) => {
            match File::open(current_dir.join(&lib.path)) {
                Ok(file) => return Ok(file),
                Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open file '{}': {}",
                        current_dir.join(&lib.path).to_string_lossy(),
                        e
                    ))
                }
                _ => {}
            }
        }
        _ => {}
    }

    // try bin defined in `Cargo.toml`
    match cargo.bin {
        // if there is only one, use it
        Some(ref bin_list) if bin_list.len() == 1 => {
            match File::open(current_dir.join(&bin_list[0].path)) {
                Ok(file) => return Ok(file),
                Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
                    return Err(format!(
                        "Could not open file '{}': {}",
                        current_dir.join(&bin_list[0].path).to_string_lossy(),
                        e
                    ))
                }
                _ => {}
            }
        }
        // if there is more than one, return an error
        Some(ref bin_list) if bin_list.len() > 1 => {
            let first = bin_list[0].path.clone();
            let paths = bin_list
                .iter()
                .skip(1)
                .map(|ref bin| bin.path.clone())
                .fold(first, |acc, path| format!("{}, {}", acc, path));
            return Err(format!("Multiple binaries found, choose one: [{}]", paths));
        }
        _ => {}
    }

    // if no entrypoint is found, return an error
    Err("No entrypoint found".to_owned())
}
