extern crate assert_cli;

use std::path::PathBuf;

use assert_cli::Assert;

#[test]
fn no_entrypoint_fail() {
    let args = ["readme", "--project-root", "tests/no-entrypoint-fail"];

    let cargo_toml = PathBuf::from("tests/no-entrypoint-fail/Cargo.toml")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let expected = format!(
        "Error: `cargo metadata` exited with an error: error: failed to parse manifest at `{cargo_toml}`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present"
    );

    Assert::main_binary()
        .with_args(&args)
        .fails()
        .and()
        .stderr()
        .is(&*expected)
        .unwrap();
}
