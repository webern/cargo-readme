//! Create README.md content from rust doc comments

#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate toml;
extern crate percent_encoding;

mod readme;
mod config;

pub use readme::generate_readme;
pub use config::get_manifest;
pub use config::project;
