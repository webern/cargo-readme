use assert_cli::Assert;

const EXPECTED_CRATE1: &str = r#"
# crate1

Test crate for cargo-readme
"#;

const EXPECTED_CRATE2: &str = r#"
# crate2

Test crate for cargo-readme
"#;

#[test]
fn workspace_crate1() {
    let args = ["readme", "--project-root", "tests/workspace/crate1"];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .and()
        .stdout()
        .is(EXPECTED_CRATE1)
        .unwrap();
}

#[test]
fn workspace_crate2() {
    let args = ["readme", "--project-root", "tests/workspace/crate2"];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .and()
        .stdout()
        .is(EXPECTED_CRATE2)
        .unwrap();
}
