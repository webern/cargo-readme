#![allow(unused)]

mod common;

#[test]
fn test() {
    let (_stdout, stderr, status) = common::chdir_cargo_readme("multiple-bin-fail", &[]);
    assert_eq!(
        "Error: Multiple binaries found, choose one: [src/entry1.rs, src/entry2.rs]",
        stderr.trim()
    );
    assert_eq!(status, 1);
}
