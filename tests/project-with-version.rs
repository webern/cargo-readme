use assert_cmd::Command;

const EXPECTED: &str = "# project-with-version

Current version: 0.1.0

A test project with a version provided.";

#[test]
fn template_with_version() {
    let args = [
        "readme",
        "--project-root",
        "tests/project-with-version",
        "--template",
        "README.tpl",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(EXPECTED);
}
