#![allow(unused)]

mod common;

const CARGO_TOML: &str = r#"
[package]
name = "multiple-bin"
version = "0.1.0"
[[bin]]
name = "entry1"
path = "src/entry1.rs"
[[bin]]
name = "entry2"
path = "src/entry2.rs"
"#;

#[test]
fn test() {
    let (_stdout, stderr, status) = common::custom_manifest_cargo_readme(CARGO_TOML, &[]);
    assert_eq!(
        "Error: Multiple binaries found, choose one: [src/entry1.rs, src/entry2.rs]",
        stderr.trim()
    );
    assert_eq!(status, 1);
}
