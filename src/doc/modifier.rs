use std::iter::Iterator;
use std::io;

use regex::Regex;

pub trait DocModify<I: Iterator> {
    fn modify_doc(self, indent_headings: bool) -> DocModifier<Self>
        where Self: Sized + Iterator<Item=io::Result<String>>
    {
        DocModifier::new(self, indent_headings)
    }
}

#[derive(PartialEq)]
enum Code {
    Rust,
    Other,
    None,
}

pub struct DocModifier<I: Iterator> {
    iter: I,
    indent_headings: bool,
    section: Code,
    re_code_rust: Regex,
    re_code_other: Regex,
}

impl<I> DocModifier<I> where I: Iterator<Item=io::Result<String>> {
    pub fn new(iter: I, indent_headings: bool) -> Self {
        // Is this code block rust?
        let re_code_rust = Regex::new(r"^```(no_run|ignore|should_panic)?$").unwrap();
        // Is this code block a language other than rust?
        let re_code_other = Regex::new(r"^```\w*$").unwrap();

        DocModifier {
            iter: iter,
            indent_headings: indent_headings,
            section: Code::None,
            re_code_rust: re_code_rust,
            re_code_other: re_code_other,
        }
    }
}

impl<I> Iterator for DocModifier<I> where I: Iterator<Item=io::Result<String>> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = match self.iter.next() {
            Some(Ok(line)) => line,
            None => return None,
            err => return err,
        };

        // Skip lines that should be hidden in docs
        while self.section == Code::Rust && line.starts_with("#") {
            line = match self.iter.next() {
                Some(Ok(line)) => line,
                None => return None,
                err => return err,
            };
        }

        // indent heading when outside code
        if self.indent_headings && self.section == Code::None && line.starts_with("#") {
            line.insert(0, '#');
        } else if self.section == Code::None && self.re_code_rust.is_match(&line) {
            self.section = Code::Rust;
            line = "```rust".to_owned();
        } else if self.section == Code::None && self.re_code_other.is_match(&line) {
            self.section = Code::Other;
        } else if self.section != Code::None && line == "```" {
            self.section = Code::None;
        }

        Some(Ok(line))
    }
}