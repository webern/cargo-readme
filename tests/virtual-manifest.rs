use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn single_member_workspace_resolves_to_that_member() {
    let args = [
        "readme",
        "--project-root",
        "tests/virtual-manifest/single",
        "--no-template",
    ];

    let expected = r#"# single-member

The only package in this workspace.

License: MIT
"#;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn multi_member_workspace_lists_the_candidates() {
    let args = [
        "readme",
        "--project-root",
        "tests/virtual-manifest/multi",
        "--no-template",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(contains(
            "Multiple workspace members found, choose one with --project-root: \
             [crates/alpha, crates/beta]",
        ));
}

#[test]
fn a_member_of_a_multi_member_workspace_still_works() {
    let args = [
        "readme",
        "--project-root",
        "tests/virtual-manifest/multi/crates/beta",
        "--no-template",
    ];

    let expected = r#"# beta

Docs for beta.

License: MIT
"#;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}
