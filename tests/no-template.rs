#![allow(unused)]

mod common;

const EXPECTED: &str =
r#"
# readme-test

Test crate for cargo-readme

## Level 1 heading should become level 2

```rust
// This is standard doc test and should be output as ```rust
let condition = true;
if condition {
    // Some conditional code here
    if condition {
        // Some nested conditional code here
    }
}
```

### Level 2 heading should become level 3

```rust
// This also should output as ```rust
```
#### Level 3 heading should become level 4

```rust
// This also should output as ```rust
```

```rust
// This should output as ```rust too
```

```rust
// And also this should output as ```rust
```

```python
# This should be on the output
```
"#;

#[test]
fn test() {
    let args = ["--no-template"];

    let (stdout, stderr, _status) = common::cargo_readme(&args);
    assert_eq!(stdout, EXPECTED.trim(), "\nError: {}", stderr);
}