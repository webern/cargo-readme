use std::io::{self, Read, BufRead, BufReader};
use std::iter::Iterator;

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

    fn extract_style_none(&mut self, line: String) -> Option<String> {
        if line.starts_with("//!") {
            self.style = DocStyle::SingleLine;
            return Some(line.split_at(3).1.trim().to_owned())
        } else if line.starts_with("/*!") {
            self.style = DocStyle::MultiLine;
            let ref_line = line.split_at(3).1.trim();
            if ref_line.len() > 0 {
                return Some(ref_line.to_owned())
            }
        }
        None
    }

    fn extract_style_single_line(&mut self, line: String) -> Option<String> {
        if line.starts_with("//!") {
            return Some(line.split_at(3).1.trim().to_owned())
        } else if line.starts_with("/*!") {
            self.style = DocStyle::MultiLine;
            let ref_line = line.split_at(3).1.trim();
            if ref_line.len() > 0 {
                return Some(ref_line.to_owned())
            }
        }
        None
    }

    fn extract_style_multi_line(&mut self, line: String) -> Option<String> {
        if line.contains("*/") {
            self.style = DocStyle::NoDoc;
            let ref_line = line.split_at(line.rfind("*/").unwrap()).0.trim();
            if ref_line.len() == 0 {
                return None
            }
            return Some(ref_line.to_owned())
        }
        Some(line)
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
                return None
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

    const INPUT: &'static str =
r#"//! first line
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

    const INPUT_MULTILINE: &'static str =
r#"/*!
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

    #[test]
    fn extract_indent_headings() {
        let expected: Vec<_> =
r#"first line
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
```"#.lines().collect();

        let mut cursor = Cursor::new(INPUT.as_bytes());
        let doc_data = super::extract(&mut cursor, true).unwrap();

        assert_eq!(doc_data, expected);
    }

    #[test]
    fn extract_no_indent_headings() {
        let expected: Vec<_> =
r#"first line
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
```"#.lines().collect();

        let mut cursor = Cursor::new(INPUT.as_bytes());
        let doc_data = super::extract(&mut cursor, false).unwrap();

        assert_eq!(doc_data, expected);
    }

    #[test]
    fn extract_multiline() {
        let expected: Vec<_> =
r#"first line
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
```"#.lines().collect();

        let mut cursor = Cursor::new(INPUT_MULTILINE.as_bytes());
        let doc_data = super::extract(&mut cursor, true).unwrap();

        assert_eq!(doc_data, expected);
    }
}