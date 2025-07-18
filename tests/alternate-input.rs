use assert_cmd::Command;

#[test]
fn alternate_input_empty_docs() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "src/no_docs.rs",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("# readme-test\n\nLicense: MIT");
}

#[test]
fn alternate_input_single_line() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "src/single_line.rs",
    ];

    let expected = r#"
# readme-test

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
fn alternate_input_a_little_bit_longer() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "src/other.rs",
    ];

    let expected = r#"
# readme-test

Test crate for cargo-readme

## Level 1 heading should become level 2

License: MIT
"#;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}
