/*!
Test crate for cargo-readme

# Level 1 heading should become level 2

```
// This is standard doc test and should be output as ```rust
# This should NOT be on the output
let condition = true;
if condition {
    // Some conditional code here
    if condition {
        // Some nested conditional code here
    }
}
```

## Level 2 heading should become level 3

```ignore
// This also should output as ```rust
```
### Level 3 heading should become level 4

```ignore
// This also should output as ```rust
```

```no_run
// This should output as ```rust too
```

```should_panic
// And also this should output as ```rust
```

```python
# This should be on the output
```
*/