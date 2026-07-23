use assert_cmd::Command;

const EXPECTED: &str = r#"[![Crates.io](https://img.shields.io/crates/v/readme-test.svg)](https://crates.io/crates/readme-test)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/cargo-readme/test?branch=master&svg=true)](https://ci.appveyor.com/project/cargo-readme/test/branch/master)
[![Build Status](https://circleci.com/gh/cargo-readme/test/tree/master.svg?style=shield)](https://circleci.com/gh/cargo-readme/test/tree/master)
[![Build Status](https://gitlab.com/cargo-readme/test/badges/master/pipeline.svg)](https://gitlab.com/cargo-readme/test/commits/master)
[![Build Status](https://travis-ci.org/cargo-readme/test.svg?branch=master)](https://travis-ci.org/cargo-readme/test)
[![Workflow Status](https://github.com/cargo-readme/test/workflows/main/badge.svg)](https://github.com/cargo-readme/test/actions?query=workflow%3A%22main%22)
[![Coverage Status](https://codecov.io/gh/cargo-readme/test/branch/master/graph/badge.svg)](https://codecov.io/gh/cargo-readme/test)
[![Coverage Status](https://coveralls.io/repos/github/cargo-readme/test/badge.svg?branch=main)](https://coveralls.io/github/cargo-readme/test?branch=main)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/cargo-readme/test.svg)](https://isitmaintained.com/project/cargo-readme/test "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/cargo-readme/test.svg)](https://isitmaintained.com/project/cargo-readme/test "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# readme-test

Test crate for cargo-readme

License: MIT
"#;

#[test]
fn badges() {
    let args = ["readme", "--project-root", "tests/badges"];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(EXPECTED);
}

const EXPECTED_LIST: &str = r#"Supported badges (add under [badges] in Cargo.toml):

  crates-io  (cargo-readme extension)
    optional: crate
  appveyor
    required: repository
    optional: branch, service
  circle-ci
    required: repository
    optional: branch, service
  gitlab
    required: repository
    optional: branch
  travis-ci
    required: repository
    optional: branch
  github  (cargo-readme extension)
    required: repository
    optional: workflow
  codecov
    required: repository
    optional: branch, service
  coveralls
    required: repository
    optional: branch, service
  is-it-maintained-issue-resolution
    required: repository
  is-it-maintained-open-issues
    required: repository
  maintenance
    required: status
"#;

#[test]
fn list_badges() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["readme", "--list-badges"])
        .assert()
        .success()
        .stdout(EXPECTED_LIST);
}
