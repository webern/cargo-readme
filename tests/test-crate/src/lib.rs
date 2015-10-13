//! Test crate for cargo-readme
//!
//! This is a test docstring for cargo-readme
//! ```
//! # This should not be on the output
//! #[this_should_be_in_the_output]
//!
//! fn function() {
//!     println!("indent level");
//!     if condition {
//!         println!("deeper indent level");
//!     }
//! }
//! ```
//! # This also should be on the output, it's a heading

#[test]
fn it_works() {
}
