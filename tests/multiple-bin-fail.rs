use assert_cmd::Command;

const EXPECTED: &str =
    "Error: Multiple binaries found, choose one: [src/entry1.rs, src/entry2.rs]\n";

#[test]
fn multiple_bin_fail() {
    let args = ["readme", "--project-root", "tests/multiple-bin-fail"];

    Command::cargo_bin("cargo-readme")
        .unwrap()
        .args(&args)
        .assert()
        .failure()
        .stderr(EXPECTED);
}
