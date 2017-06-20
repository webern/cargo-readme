use std::iter::{Iterator, IntoIterator};

use super::DocStyle;
use super::transform::DocTransform;

pub trait DocExtract<I: Iterator<Item = String>> {
    fn extract_doc(self) -> DocExtractor<I>
    where
        Self: Sized + IntoIterator<IntoIter = I, Item = String>,
    {
        DocExtractor::new(self)
    }
}

// allow calling `extract_doc` from a `Vec<String>`
impl<I: Iterator<Item = String>> DocExtract<I> for Vec<String> {}

impl<I: Iterator<Item = String>> DocTransform for DocExtractor<I> {}

pub struct DocExtractor<I: Iterator> {
    iter: I,
    style: DocStyle,
}

impl<I: Iterator<Item = String>> DocExtractor<I> {
    pub fn new<J: IntoIterator<IntoIter = I, Item = String>>(iter: J) -> Self {
        DocExtractor {
            iter: iter.into_iter(),
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

impl<I: Iterator<Item = String>> Iterator for DocExtractor<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        while result.is_none() {
            let line = match self.iter.next() {
                Some(line) => line,
                None => break,
            };

            result = match self.style {
                DocStyle::NoDoc => self.extract_style_none(line),
                DocStyle::SingleLine => self.extract_style_single_line(line),
                DocStyle::MultiLine => self.extract_style_multi_line(line),
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SINGLELINE: &str = r#"//! first line
//!
//! ```
//! let rust_code = "safe";
//! ```
//!
//! ```C
//! int i = 0; // no rust code
//! ```"#;

    const INPUT_MULTILINE: &str = r#"/*!
first line

```
let rust_code = "safe";
```

```C
int i = 0; // no rust code
```
*/"#;

    const EXPECTED: &str = r#"first line

```
let rust_code = "safe";
```

```C
int i = 0; // no rust code
```"#;

    #[test]
    fn single_line() {
        let input: Vec<_> = INPUT_SINGLELINE.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED.lines().collect();

        let result: Vec<_> = DocExtractor::new(input).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn multi_line() {
        let input: Vec<_> = INPUT_MULTILINE.lines().map(|x| x.to_owned()).collect();
        let expected: Vec<_> = EXPECTED.lines().collect();

        let result: Vec<_> = DocExtractor::new(input).collect();

        assert_eq!(result, expected);
    }
}
