//! Create README.md content from rust doc comments

#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate toml;
extern crate percent_encoding;

#[cfg(test)]
#[macro_use] mod test_macros;

mod readme;
pub mod manifest;
mod config;

pub use readme::generate_readme;
