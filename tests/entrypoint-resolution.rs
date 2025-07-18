use assert_cmd::Command;

#[test]
fn entrypoint_resolution_main() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/main",
        "--no-title",
        "--no-license",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("main");
}

#[test]
fn entrypoint_resolution_lib() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/lib",
        "--no-title",
        "--no-license",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("lib");
}

#[test]
fn entrypoint_resolution_cargo_lib() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/cargo-lib",
        "--no-title",
        "--no-license",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("cargo lib");
}

#[test]
fn entrypoint_resolution_cargo_bin() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/cargo-bin",
        "--no-title",
        "--no-license",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("cargo bin");
}
