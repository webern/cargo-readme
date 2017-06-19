use std::iter::{Iterator, IntoIterator};

use regex::Regex;

pub trait DocTransform {
    fn transform_doc(self, indent_headings: bool) -> DocTransformer<Self>
    where
        Self: Sized + Iterator<Item = String>,
    {
        DocTransformer::new(self, indent_headings)
    }
}

#[derive(PartialEq)]
enum Code {
    Rust,
    Other,
    None,
}

pub struct DocTransformer<I: Iterator> {
    iter: I,
    indent_headings: bool,
    section: Code,
    re_code_rust: Regex,
    re_code_other: Regex,
}

impl<I: Iterator<Item = String>> DocTransformer<I> {
    pub fn new<J: IntoIterator<IntoIter=I, Item=String>>(iter: J, indent_headings: bool) -> Self {
        // Is this code block rust?
        let re_code_rust = Regex::new(r"^```(no_run|ignore|should_panic)?$").unwrap();
        // Is this code block a language other than rust?
        let re_code_other = Regex::new(r"^```\w*$").unwrap();

        DocTransformer {
            iter: iter.into_iter(),
            indent_headings: indent_headings,
            section: Code::None,
            re_code_rust: re_code_rust,
            re_code_other: re_code_other,
        }
    }
}

impl<I> Iterator for DocTransformer<I>
where
    I: Iterator<Item = String>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = match self.iter.next() {
            Some(line) => line,
            None => return None,
        };

        // Skip lines that should be hidden in docs
        while self.section == Code::Rust && line.starts_with("#") {
            line = match self.iter.next() {
                Some(line) => line,
                None => return None,
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

        Some(line)
    }
}


#[cfg(test)]
mod tests {
    use super::DocTransformer;

    const INPUT: &'static str = r#"first line
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
```"#;

    const EXPECT_INDENT_HEADINGS: &str = r#"first line
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

    const EXPECT_NO_INDENT_HEADINGS: &str = r#"first line
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

    #[test]
    fn indent_headings() {
        let input: Vec<_> = INPUT.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECT_INDENT_HEADINGS.lines().collect();

        let result: Vec<_> = DocTransformer::new(input, true).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn no_indent_headings() {
        let input: Vec<_> = INPUT.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECT_NO_INDENT_HEADINGS.lines().collect();

        let result: Vec<_> = DocTransformer::new(input, false).collect();

        assert_eq!(result, expected);
    }
}
