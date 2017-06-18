use std::io::{self, Read, BufRead, BufReader};
use std::iter::Iterator;

use super::modifier::DocModify;

impl<R: Read> DocModify<DocExtractor<R>> for DocExtractor<R> {}

pub enum DocStyle {
    NoDoc,
    SingleLine,
    MultiLine,
}

pub struct DocExtractor<R: Read> {
    reader: BufReader<R>,
    style: DocStyle,
}

impl<R: Read> DocExtractor<R> {
    pub fn new(reader: R) -> Self {
        DocExtractor {
            reader: BufReader::new(reader),
            style: DocStyle::NoDoc,
        }
    }

    // normalizes a line by stripping the "//!" or "/*!" from it and a single whitespace
    fn normalize_line(&self, mut line: String) -> String {
        if line.trim().len() <= 3 {
            line.clear();
            line
        } else {
            // if the first character after the comment is " ", remove it
            let split_at = if line.find(" ") == Some(3) { 4 } else { 3 };
            line.split_at(split_at).1.trim_right().to_owned()
        }
    }

    fn extract_style_none(&mut self, line: String) -> Option<String> {
        if line.starts_with("//!") {
            self.style = DocStyle::SingleLine;
            return Some(self.normalize_line(line));
        } else if line.starts_with("/*!") {
            self.style = DocStyle::MultiLine;
            let line = self.normalize_line(line);
            if line.len() > 0 {
                return Some(line);
            }
        }
        None
    }

    fn extract_style_single_line(&mut self, line: String) -> Option<String> {
        if line.starts_with("//!") {
            return Some(self.normalize_line(line));
        } else if line.starts_with("/*!") {
            self.style = DocStyle::MultiLine;
            let line = self.normalize_line(line);
            if line.len() > 0 {
                return Some(line);
            }
        }
        None
    }

    fn extract_style_multi_line(&mut self, line: String) -> Option<String> {
        if line.contains("*/") {
            self.style = DocStyle::NoDoc;
            let ref_line = line.split_at(line.rfind("*/").unwrap()).0.trim_right();
            if ref_line.len() == 0 {
                return None;
            }
            return Some(ref_line.to_owned());
        }
        Some(line.trim_right().to_owned())
    }
}

impl<R: Read> Iterator for DocExtractor<R> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let mut bytes_read: usize;

        while result.is_none() {
            let mut buffer = String::new();

            bytes_read = match self.reader.read_line(&mut buffer) {
                Ok(bytes_read) => bytes_read,
                Err(e) => return Some(Err(e)),
            };

            if bytes_read == 0 {
                return None;
            }

            result = match self.style {
                DocStyle::NoDoc => self.extract_style_none(buffer),
                DocStyle::SingleLine => self.extract_style_single_line(buffer),
                DocStyle::MultiLine => self.extract_style_multi_line(buffer),
            };
        }

        result.map(|x| Ok(x))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::DocExtractor;
    use super::DocModify;

    const INPUT: &'static str = r#"//! first line
//! ```
//! let rust_code = "will show";
//! # let binding = "won't show";
//! ```
//! # heading
//! ```no_run
//! let no_run = true;
//! ```
//! ```ignore
//! let ignore = true;
//! ```
//! ```should_panic
//! let should_panic = true;
//! ```
//! # heading
//! ```C
//! int i = 0; // no rust code
//! ```
use std::any::Any;

fn main() {}"#;

    const INPUT_MULTILINE: &'static str = r#"/*!
first line
```
let rust_code = "will show";
# let binding = "won't show";
```
# heading
```no_run
let no_run = true;
```
```ignore
let ignore = true;
```
```should_panic
let should_panic = true;
```
# heading
```C
int i = 0; // no rust code
```
*/
use std::any::Any;

fn main() {}"#;

    const EXPECT_INDENT_HEADING: &str = r#"first line
```rust
let rust_code = "will show";
```
## heading
```rust
let no_run = true;
```
```rust
let ignore = true;
```
```rust
let should_panic = true;
```
## heading
```C
int i = 0; // no rust code
```"#;

    const EXPECT_NO_INDENT_HEADING: &str = r#"first line
```rust
let rust_code = "will show";
```
# heading
```rust
let no_run = true;
```
```rust
let ignore = true;
```
```rust
let should_panic = true;
```
# heading
```C
int i = 0; // no rust code
```"#;

    fn extract_doc(cursor: Cursor<&[u8]>, indent_headings: bool) -> Vec<String> {
        let mut result = Vec::new();
        let iter = DocExtractor::new(cursor).modify_doc(indent_headings);
        for line in iter {
            let line = line.unwrap();
            result.push(line);
        }
        result
    }

    #[test]
    fn extract_indent_headings() {
        let expected: Vec<_> = EXPECT_INDENT_HEADING.lines().collect();

        let cursor = Cursor::new(INPUT.as_bytes());
        let doc_data = extract_doc(cursor, true);

        assert_eq!(doc_data, expected);
    }

    #[test]
    fn extract_no_indent_headings() {
        let expected: Vec<_> = EXPECT_NO_INDENT_HEADING.lines().collect();

        let cursor = Cursor::new(INPUT.as_bytes());
        let doc_data = extract_doc(cursor, false);

        assert_eq!(doc_data, expected);
    }

    #[test]
    fn extract_multiline_indent_headings() {
        let expected: Vec<_> = EXPECT_INDENT_HEADING.lines().collect();

        let cursor = Cursor::new(INPUT_MULTILINE.as_bytes());
        let doc_data = extract_doc(cursor, true);

        assert_eq!(doc_data, expected);
    }

    #[test]
    fn extract_multiline_no_indent_headings() {
        let expected: Vec<_> = EXPECT_NO_INDENT_HEADING.lines().collect();

        let cursor = Cursor::new(INPUT_MULTILINE.as_bytes());
        let doc_data = extract_doc(cursor, false);

        assert_eq!(doc_data, expected);
    }
}
