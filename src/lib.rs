extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod readme;
pub mod cargo_info;

pub use readme::generate_readme;