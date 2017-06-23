//! Transform code blocks from rustdoc into markdown
//!
//! Rewrite code block start tags, changing rustdoc into equivalent in markdown:
//! - "```", "```no_run", "```ignore" and "```should_panic" are converted to "```rust"
//! - markdown heading are indentend to be one level lower, so the crate name is at the top level

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
    pub fn new<J: IntoIterator<IntoIter = I, Item = String>>(
        iter: J,
        indent_headings: bool,
    ) -> Self {
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
        while self.section == Code::Rust && line.starts_with("# ") {
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

    const INPUT_HIDDEN_LINE: &str = r#"
```
#[visible]
let visible = "visible";
# let hidden = "hidden";
```"#;

    const EXPECTED_HIDDEN_LINE: &str = r#"
```rust
#[visible]
let visible = "visible";
```"#;

    #[test]
    fn hide_line_in_rust_code_block() {
        let input: Vec<_> = INPUT_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();

        let result: Vec<_> = DocTransformer::new(input, true).collect();

        assert_eq!(result, expected);
    }

    const INPUT_NOT_HIDDEN_LINE: &str = r#"
```
let visible = "visible";
# let hidden = "hidden";
```

```python
# this line is visible
visible = True
```"#;

    const EXPECTED_NOT_HIDDEN_LINE: &str = r#"
```rust
let visible = "visible";
```

```python
# this line is visible
visible = True
```"#;

    #[test]
    fn do_not_hide_line_in_code_block() {
        let input: Vec<_> = INPUT_NOT_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED_NOT_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();

        let result: Vec<_> = DocTransformer::new(input, true).collect();

        assert_eq!(result, expected);
    }

const INPUT_RUST_CODE_BLOCK: &'static str = r#"
```
let block = "simple code block";
```

```no_run
let run = false;
```

```ignore
let ignore = true;
```

```should_panic
panic!("at the disco");
```

```C
int i = 0; // no rust code
```"#;

    const EXPECTED_RUST_CODE_BLOCK: &str = r#"
```rust
let block = "simple code block";
```

```rust
let run = false;
```

```rust
let ignore = true;
```

```rust
panic!("at the disco");
```

```C
int i = 0; // no rust code
```"#;

    #[test]
    fn transform_rust_code_block() {
        let input: Vec<_> = INPUT_RUST_CODE_BLOCK.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED_RUST_CODE_BLOCK.lines().map(|x| x.to_owned()).collect();

        let result: Vec<_> = DocTransformer::new(input, true).collect();

        assert_eq!(result, expected);
    }

    const INPUT_INDENT_HEADINGS: &'static str = r#"
# heading 1
some text
## heading 2
some other text
"#;

    const EXPECTED_INDENT_HEADINGS: &str = r#"
## heading 1
some text
### heading 2
some other text
"#;

    #[test]
    fn indent_markdown_headings() {
        let input: Vec<_> = INPUT_INDENT_HEADINGS.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED_INDENT_HEADINGS.lines().collect();

        let result: Vec<_> = DocTransformer::new(input, true).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn do_not_indent_markdown_headings() {
        let input: Vec<_> = INPUT_INDENT_HEADINGS.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = INPUT_INDENT_HEADINGS.lines().collect();

        let result: Vec<_> = DocTransformer::new(input, false).collect();

        assert_eq!(result, expected);
    }
}
