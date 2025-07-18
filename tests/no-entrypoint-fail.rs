use assert_cmd::Command;

const EXPECTED: &str = "Error: No entrypoint found";

#[test]
fn no_entrypoint_fail() {
    let args = ["readme", "--project-root", "tests/no-entrypoint-fail"];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(EXPECTED);
}
