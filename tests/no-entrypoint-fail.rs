extern crate assert_cli;

use assert_cli::Assert;

const EXPECTED: &str = "Error: No entrypoint found";

#[test]
fn test() {
    let args = [
        "readme",
        "--project-root", "tests/no-entrypoint-fail",
    ];

    Assert::main_binary()
        .with_args(&args)
        .fails()
        .prints_error(EXPECTED)
        .unwrap();
}
