#![allow(unused)]

mod common;

#[test]
fn empty_docs() {
    let args = ["--no-template", "--input", "src/no_docs.rs"];

    let (stdout, stderr, _status) = common::cargo_readme(&args);

    assert_eq!(stdout, "# readme-test\n\nLicense: MIT", "\nError: {}", stderr);
}

#[test]
fn single_line() {
    let args = ["--no-template", "--input", "src/single_line.rs"];

    let expected = r#"
# readme-test

Test crate for cargo-readme

License: MIT
"#;

    let (stdout, stderr, _status) = common::cargo_readme(&args);
    assert_eq!(stdout, expected.trim(), "\nError: {}", stderr);
}

#[test]
fn a_little_bit_longer() {
    let args = ["--no-template", "--input", "src/other.rs"];

    let expected = r#"
# readme-test

Test crate for cargo-readme

## Level 1 heading should become level 2

License: MIT
"#;

    let (stdout, stderr, _status) = common::cargo_readme(&args);
    assert_eq!(stdout, expected.trim(), "\nError: {}", stderr);
}
