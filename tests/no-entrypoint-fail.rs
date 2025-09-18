use assert_cmd::Command;

const EXPECTED: &str = "Error: No entrypoint found\n";

#[test]
fn no_entrypoint_fail() {
    let args = ["readme", "--project-root", "tests/no-entrypoint-fail"];

    Command::cargo_bin("cargo-readme")
        .unwrap()
        .args(&args)
        .assert()
        .failure()
        .stderr(EXPECTED);
}
