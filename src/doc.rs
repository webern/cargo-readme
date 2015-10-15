use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

const SRC_RUST: &'static str = "SRC_RUST";
const SRC_OTHER: &'static str = "SRC_OTHER";
const SRC_DOC: &'static str = "SRC_DOC";

pub fn read<T: Read>(source: &mut T) -> Vec<String> {
    let reader = BufReader::new(source);

    let re_code_rust = Regex::new(r"^//! ```(no_run|ignore|should_panic)?$").unwrap();
    let re_code_other = Regex::new(r"//! ```\w+").unwrap();

    let mut section = SRC_DOC;

    reader.lines().filter_map(|line| {
        let line = line.unwrap();
        if line.starts_with("//!") {

            if  section == SRC_DOC && re_code_rust.is_match(&line) {
                section = SRC_RUST;

                return Some("```rust".to_owned());
            }
            else if section == SRC_DOC && re_code_other.is_match(&line) {
                section = SRC_OTHER;
            }
            else if section != SRC_DOC && line == "//! ```" {
                section = SRC_DOC;

                return Some("```".to_owned());
            }

            if section == SRC_RUST && line.starts_with("//! # ") {
                return None;
            }

            // Remove leading '//!' before returning the line
            if line.len() == 3 {
                Some("".to_owned())
            }
            else {
                Some(line[4..].to_owned())
            }
        }
         else {
            return None
        }
    })
    .collect()
}
