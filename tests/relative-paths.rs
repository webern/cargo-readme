use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use std::path::PathBuf;

const TEMPLATE_EXPECTED: &str = r#"# readme-test

Other readme template.

Test crate for cargo-readme
"#;

#[test]
fn input_is_relative_to_cwd() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "tests/test-project/src/single_line.rs",
    ];

    let expected = r#"# readme-test

Test crate for cargo-readme

License: MIT
"#;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn input_relative_to_project_root_is_not_found() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "src/single_line.rs",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(contains("Could not open file 'src/single_line.rs'"));
}

#[test]
fn template_is_relative_to_cwd() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-badges",
        "--input",
        "tests/test-project/src/single_line.rs",
        "--template",
        "tests/test-project/NOTITLE.tpl",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(TEMPLATE_EXPECTED);
}

#[test]
fn template_relative_to_project_root_is_not_found() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-badges",
        "--template",
        "NOTITLE.tpl",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(contains("Could not open template file 'NOTITLE.tpl'"));
}

/// The default template is looked up next to `Cargo.toml`, so `--project-root` stays its base.
#[test]
fn default_template_is_relative_to_project_root() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-badges",
        "--input",
        "tests/test-project/src/single_line.rs",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(contains("Some text here"));
}

#[test]
fn output_is_relative_to_cwd() {
    let cwd = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("output-is-relative-to-cwd");
    fs::create_dir_all(&cwd).unwrap();
    let written = cwd.join("README.md");
    fs::remove_file(&written).ok();

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test-project");
    let args = [
        "readme",
        "--project-root",
        project_root.to_str().unwrap(),
        "--no-template",
        "--no-badges",
        "--output",
        "README.md",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(&cwd)
        .args(args)
        .assert()
        .success();

    assert!(written.is_file());
    assert!(!project_root.join("README.md").exists());
}
