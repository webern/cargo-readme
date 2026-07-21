use assert_cmd::Command;

// A plain Markdown file (the kind embedded with `#![doc = include_str!(...)]`) is not made of doc
// comments, so extraction finds nothing and the body comes out empty.
#[test]
fn markdown_input_without_flag_is_empty() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--input",
        "README.rustdoc.md",
    ];

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout("# readme-test\n\nLicense: MIT\n");
}

// With `--no-comment-extraction` the same file is processed verbatim: headings are indented, code
// fences become `rust`, and hidden `# ` lines are dropped, including indented ones.
#[test]
fn markdown_input_with_flag_is_processed() {
    let args = [
        "readme",
        "--project-root",
        "tests/test-project",
        "--no-template",
        "--no-badges",
        "--no-comment-extraction",
        "--input",
        "README.rustdoc.md",
    ];

    let expected = r#"# readme-test

Test crate for cargo-readme

This file is plain Markdown, meant to be embedded with `#![doc = include_str!(...)]`,
so `cargo readme` must process it verbatim rather than extracting doc comments.

## Examples

```rust
let visible = "visible";
```

License: MIT
"#;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}
