extern crate assert_cli;

use assert_cli::Assert;

const EXPECTED: &str = r#"
[![Build Status](https://travis-ci.com/cargo-readme/test.svg?branch=master)](https://travis-ci.com/cargo-readme/test)

# readme-test

Test crate for cargo-readme

License: MIT
"#;

#[test]
fn badges() {
    let args = ["readme", "--project-root", "tests/travis-com"];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .and()
        .stdout()
        .is(EXPECTED)
        .unwrap();
}
