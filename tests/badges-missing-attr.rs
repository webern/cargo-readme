use assert_cmd::Command;

const EXPECTED: &str = "Error: badge `coveralls` is missing required attribute `repository`\n";

#[test]
fn badges_missing_attr_fail() {
    let args = ["readme", "--project-root", "tests/badges-missing-attr"];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(EXPECTED);
}
