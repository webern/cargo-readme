use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::time::{self, SystemTime};
use std::path::PathBuf;
use std::process::{Command, Output};

pub fn cargo_readme(args: &[&str]) -> (String, String, i32) {
    let root = env::current_dir().expect("no current dir"); // I hope you're running "cargo test" from the project root

    let test_project = root.join("tests/test-project");
    let cargo_readme = root.join("target/debug/cargo-readme");

    let mut command = Command::new(cargo_readme);
    command.current_dir(test_project);
    command.arg("readme");
    command.args(args);

    let result = command.output().expect("error executing cargo readme");

    parse_result(result)
}

pub fn custom_manifest_cargo_readme(manifest: &str, args: &[&str]) -> (String, String, i32) {
    let path = create_cargo_toml(manifest);

    let root = env::current_dir().expect("no current dir"); // I hope you're running "cargo test" from the project root

    let test_project = path.parent().unwrap();
    let cargo_readme = root.join("target/debug/cargo-readme");

    let mut command = Command::new(cargo_readme);
    command.current_dir(test_project);
    command.arg("readme");
    command.args(args);

    let result = command.output().expect("error executing cargo readme");

    parse_result(result)
}

fn create_cargo_toml(manifest: &str) -> PathBuf {
    let now = SystemTime::now();
    let t = now.duration_since(time::UNIX_EPOCH).expect("SystemTime failed");
    let secs = t.as_secs();
    let nanos = t.subsec_nanos();

    let mut dir = env::temp_dir();
    dir.push(format!("cargo-readme/test/{}{}", secs, nanos));
    fs::create_dir_all(&dir).expect("cannot create temporary project dir");

    let mut path = dir;
    path.push("Cargo.toml");
    let mut cargo_toml = File::create(&path).expect("cannot create Cargo.toml");
    cargo_toml.write_all(manifest.as_bytes()).expect("cannot write to Cargo.toml");

    path
}

pub fn parse_result(result: Output) -> (String, String, i32) {
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