//! Create README.md content from rust doc comments

#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate toml;

#[cfg(test)]
#[macro_use] mod test_macros;

mod readme;
pub mod manifest;

pub use readme::generate_readme;
