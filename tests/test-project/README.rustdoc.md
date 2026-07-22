Test crate for cargo-readme

This file is plain Markdown, meant to be embedded with `#![doc = include_str!(...)]`,
so `cargo readme` must process it verbatim rather than extracting doc comments.

# Examples

```
let visible = "visible";
# let hidden = "hidden";
    # let hidden_but_indented = "also hidden";
```
