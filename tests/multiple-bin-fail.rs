extern crate assert_cli;

use std::path::PathBuf;

use assert_cli::Assert;

#[test]
fn multiple_bin_fail() {
    let args = ["readme", "--project-root", "tests/multiple-bin-fail"];

    let base = PathBuf::from("tests/multiple-bin-fail")
        .canonicalize()
        .unwrap();
    let entry1 = base.join("src/entry1.rs").to_str().unwrap().to_string();
    let entry2 = base.join("src/entry2.rs").to_str().unwrap().to_string();
    let expected = format!("Error: Multiple binaries found, choose one: [{entry1}, {entry2}]");

    Assert::main_binary()
        .with_args(&args)
        .fails()
        .and()
        .stderr()
        .is(&*expected)
        .unwrap();
}
