#![allow(unused)]

mod common;

const CARGO_TOML: &str = r#"
[package]
name = "no-entrypoint"
version = "0.1.0"
"#;

#[test]
fn test() {
    let (_stdout, stderr, status) = common::custom_manifest_cargo_readme(CARGO_TOML, &[]);
    assert_eq!("Error: No entrypoint found", stderr.trim());
    assert_eq!(status, 1);
}