#![allow(unused)]

mod common;

#[test]
fn test() {
    let (_stdout, stderr, status) = common::chdir_cargo_readme("no-entrypoint-fail", &[]);
    assert_eq!("Error: No entrypoint found", stderr.trim());
    assert_eq!(status, 1);
}
