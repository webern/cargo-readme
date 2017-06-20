use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::time::{self, SystemTime};
use std::path::PathBuf;
use std::process::{Command, Output};

pub fn cargo_readme(args: &[&str]) -> (String, String, i32) {
    chdir_cargo_readme("test-project", args)
}

pub fn chdir_cargo_readme(chdir: &str, args: &[&str]) -> (String, String, i32) {
    // I hope you're running "cargo test" from the project root
    let root = env::current_dir().expect("no current dir");

    let test_project = root.join("tests").join(chdir);
    let cargo_readme = root.join("target/debug/cargo-readme");

    let mut command = Command::new(cargo_readme);
    command.current_dir(test_project);
    command.arg("readme");
    command.args(args);

    let result = command.output().expect("error executing cargo readme");

    parse_result(result)
}

fn parse_result(result: Output) -> (String, String, i32) {
    let stdout = String::from_utf8(result.stdout)
        .expect("invalid utf-8 stdout")
        .trim()
        .to_owned();

    let stderr = String::from_utf8(result.stderr)
        .expect("invalid utf-8 stderr")
        .trim()
        .to_owned();

    let status = result.status.code().expect("test process interrupted");

    (stdout, stderr, status)
}
