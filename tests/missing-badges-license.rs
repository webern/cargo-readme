use assert_cmd::Command;

const EXPECTED: &str = "\n\n# missing-badges-license\n\nTest crate for cargo-readme\n\nLicense: \n";

const EXPECTED_WARNINGS: &str =
    "Warn: `{{badges}}` was found in template but no badges were provided
Warn: `{{license}}` was found in template but no license was provided
";

#[test]
fn missing_badges_license() {
    let args = ["readme", "--project-root", "tests/missing-badges-license"];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(EXPECTED)
        .stderr(EXPECTED_WARNINGS);
}
