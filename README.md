# cargo-readme

Generate README.md from doc comments.

## Installation

Just clone this repository, run `cargo build --release` and add `target/release/cargo-readme`
somewhere in your path.

## About

This cargo subcommand will extract documentation from your crate's doc comments
that you can use to populate its README.md.

For example, if your crate has the following doc comments at `lib.rs`

```rust
//! This is my awesome crate
//!
//! Here goes some other description of what it is and what is does
//!
//! # Examples
//! ```
//! fn sum2(n1: i32, n2: i32) -> i32 {
//!   n1 + n2
//! }
//! # assert_eq!(4, sum2(2, 2));
//! ```
```

you may want to use it as your `README.md` content (without rust's doc comments specific stuff, obviously)
so you don't have to maintain the same documentation in the different places.

Using `cargo-readme`, you write the documentation as doc comments, fill README.md with it and
you can be sure that the examples are correct.

The result would look like:

    # crate-name

    This is my awesome crate

    Here goes some other description of what it is and what is does

    ## Examples
    ```rust
    fn sum2(n1: i32, n2: i32) -> i32 {
      n1 + n2
    }
    ```

You may have noticed that `# Examples` became `## Examples`. This is intentional (and can be disabled)
so in README.md the first heading can be your crate name.

Also, the crate name was automatically added (can be disabled too). It is read
from `Cargo.toml` so you just need to have them there. License can be read from
`Cargo.toml`, but it's opt-in.

If you have additional information that does not fit in doc comments, you can use
a template. To do so, just create a file called `README.tpl` in the same directory
as `Cargo.toml` with the following content

```
Your crate's badges here

{{readme}}

Some additional info here
```

The output will look like this

    # crate-name

    Your crate's badges here

    This is my awesome crate

    Here goes some other description of what it is and what is does

    ## Examples
    ```rust
    fn sum2(n1: i32, n2: i32) -> i32 {
      n1 + n2
    }
    ```

    Some additional info here

You can override the displaying of your crate's name and license using `{{crate}}`
and `{{license}}`.
