use assert_cmd::Command;

const EXPECTED_TEMPLATE: &str = r#"[![Workflow Status](https://github.com/cargo-readme/test/workflows/main/badge.svg)](https://github.com/cargo-readme/test/actions?query=workflow%3A%22main%22)

# workspace-member

Current version: 1.2.3

A test project using workspace inheritance.

License: MIT
"#;

const EXPECTED_NO_TEMPLATE: &str = r#"[![Workflow Status](https://github.com/cargo-readme/test/workflows/main/badge.svg)](https://github.com/cargo-readme/test/actions?query=workflow%3A%22main%22)

# workspace-member

A test project using workspace inheritance.

License: MIT
"#;

#[test]
fn workspace_inheritance_with_template() {
    let args = [
        "readme",
        "--project-root",
        "tests/workspace-inheritance/member",
        "--template",
        "README.tpl",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(EXPECTED_TEMPLATE);
}

#[test]
fn workspace_inheritance_without_template() {
    let args = [
        "readme",
        "--project-root",
        "tests/workspace-inheritance/member",
        "--no-template",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(EXPECTED_NO_TEMPLATE);
}
