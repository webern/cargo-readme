//! Transform code blocks from rustdoc into markdown
//!
//! Rewrite code block start tags, changing rustdoc into equivalent in markdown:
//! - "```", "```no_run", "```ignore" and "```should_panic" are converted to "```rust"
//! - markdown heading are indentend to be one level lower, so the crate name is at the top level

use std::iter::{Iterator, IntoIterator};

use regex::Regex;

const REGEX_CODE_RUST: &'static str = r"^```(rust|((rust,)?(no_run|ignore|should_panic)))?$";
const REGEX_CODE_TEXT: &'static str = r"^```text$";
const REGEX_CODE_OTHER: &'static str = r"^```\w[\w,\+]*$";

/// Process and concatenate the doc lines into a single String
///
/// The processing transforms doc tests into regular rust code blocks and optionally indent the
/// markdown headings in order to leave the top heading to the crate name
pub fn process_docs<S: Into<String>, L: Into<Vec<S>>>(lines: L, indent_headings: bool) -> Vec<String> {
    lines.into().into_iter()
        .process_docs(indent_headings)
}

pub struct Processor {
    section: Section,
    indent_headings: bool,
    re_code_rust: Regex,
    re_code_text: Regex,
    re_code_other: Regex,
}

impl Processor {
    pub fn new(indent_headings: bool) -> Self {
        // Is this code block rust?
        let re_code_rust = Regex::new(REGEX_CODE_RUST).unwrap();
        // Is this code block just text?
        let re_code_text = Regex::new(REGEX_CODE_TEXT).unwrap();
        // Is this code block a language other than rust?
        let re_code_other = Regex::new(REGEX_CODE_OTHER).unwrap();

        Processor {
            section: Section::None,
            indent_headings: indent_headings,
            re_code_rust: re_code_rust,
            re_code_text: re_code_text,
            re_code_other: re_code_other,
        }
    }

    pub fn process_line(&mut self, mut line: String) -> Option<String> {
        // Skip lines that should be hidden in docs
        if self.section == Section::CodeRust && line.starts_with("# ") {
            return None;
        }

        // indent heading when outside code
        if self.indent_headings && self.section == Section::None && line.starts_with("#") {
            line.insert(0, '#');
        } else if self.section == Section::None && self.re_code_rust.is_match(&line) {
            self.section = Section::CodeRust;
            line = "```rust".to_owned();
        } else if self.section == Section::None && self.re_code_text.is_match(&line) {
            self.section = Section::CodeOther;
            line = "```".to_owned();
        } else if self.section == Section::None && self.re_code_other.is_match(&line) {
            self.section = Section::CodeOther;
        } else if self.section != Section::None && line == "```" {
            self.section = Section::None;
        }

        Some(line)
    }
}

#[derive(PartialEq)]
enum Section {
    CodeRust,
    CodeOther,
    None,
}

pub trait DocProcess<S: Into<String>> {
    fn process_docs(self, indent_headings: bool) -> Vec<String>
    where
        Self: Sized + Iterator<Item = S>,
    {
        let mut p = Processor::new(indent_headings);
        self.into_iter()
            .filter_map(|line| p.process_line(line.into()))
            .collect()
    }
}

impl<S: Into<String>, I: Iterator<Item = S>> DocProcess<S> for I {}


#[cfg(test)]
mod tests {
    use super::process_docs;

    const INPUT_HIDDEN_LINE: &[&str] = &[
        "```",
        "#[visible]",
        "let visible = \"visible\";",
        "# let hidden = \"hidden\";",
        "```",
    ];

    const EXPECTED_HIDDEN_LINE: &[&str] = &[
        "```rust",
        "#[visible]",
        "let visible = \"visible\";",
        "```",
    ];

    #[test]
    fn hide_line_in_rust_code_block() {
        // let input: Vec<&str> = INPUT_HIDDEN_LINE.into_iter().collect();
        // let expected: Vec<_> = EXPECTED_HIDDEN_LINE.into_iter().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_HIDDEN_LINE, true);

        assert_eq!(result, EXPECTED_HIDDEN_LINE);
    }

    const INPUT_NOT_HIDDEN_LINE: &[&str] = &[
        "```",
        "let visible = \"visible\";",
        "# let hidden = \"hidden\";",
        "```",
        "",
        "```python",
        "# this line is visible",
        "visible = True",
        "```",
    ];

    const EXPECTED_NOT_HIDDEN_LINE: &[&str] = &[
        "```rust",
        "let visible = \"visible\";",
        "```",
        "",
        "```python",
        "# this line is visible",
        "visible = True",
        "```",
    ];

    #[test]
    fn do_not_hide_line_in_code_block() {
        // let input: Vec<_> = INPUT_NOT_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = EXPECTED_NOT_HIDDEN_LINE.lines().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_NOT_HIDDEN_LINE, true);

        assert_eq!(result, EXPECTED_NOT_HIDDEN_LINE);
    }

    const INPUT_RUST_CODE_BLOCK: &[&str] = &[
        "```",
        "let block = \"simple code block\";",
        "```",
        "",
        "```no_run",
        "let run = false;",
        "```",
        "",
        "```ignore",
        "let ignore = true;",
        "```",
        "",
        "```should_panic",
        "panic!(\"at the disco\");",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    const EXPECTED_RUST_CODE_BLOCK: &[&str] = &[
        "```rust",
        "let block = \"simple code block\";",
        "```",
        "",
        "```rust",
        "let run = false;",
        "```",
        "",
        "```rust",
        "let ignore = true;",
        "```",
        "",
        "```rust",
        "panic!(\"at the disco\");",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    #[test]
    fn transform_rust_code_block() {
        // let input: Vec<_> = INPUT_RUST_CODE_BLOCK.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = EXPECTED_RUST_CODE_BLOCK.lines().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_RUST_CODE_BLOCK, true);

        assert_eq!(result, EXPECTED_RUST_CODE_BLOCK);
    }

    const INPUT_RUST_CODE_BLOCK_RUST_PREFIX: &[&str] = &[
        "```rust",
        "let block = \"simple code block\";",
        "```",
        "",
        "```rust,no_run",
        "let run = false;",
        "```",
        "",
        "```rust,ignore",
        "let ignore = true;",
        "```",
        "",
        "```rust,should_panic",
        "panic!(\"at the disco\");",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    #[test]
    fn transform_rust_code_block_with_prefix() {
        // let input: Vec<_> = INPUT_RUST_CODE_BLOCK_RUST_PREFIX.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = EXPECTED_RUST_CODE_BLOCK.lines().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_RUST_CODE_BLOCK_RUST_PREFIX, true);

        assert_eq!(result, EXPECTED_RUST_CODE_BLOCK);
    }

    const INPUT_TEXT_BLOCK: &[&str] = &[
        "```text",
        "this is text",
        "```",
    ];

    const EXPECTED_TEXT_BLOCK: &[&str] = &[
        "```",
        "this is text",
        "```",
    ];

    #[test]
    fn transform_text_block() {
        // let input: Vec<_> = INPUT_TEXT_BLOCK.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = EXPECTED_TEXT_BLOCK.lines().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_TEXT_BLOCK, true);

        assert_eq!(result, EXPECTED_TEXT_BLOCK);
    }

    const INPUT_OTHER_CODE_BLOCK_WITH_SYMBOLS: &[&str] = &[
        "```html,django",
        "{% if True %}True{% endif %}",
        "```",
        "",
        "```html+django",
        "{% if True %}True{% endif %}",
        "```",
    ];

    #[test]
    fn transform_other_code_block_with_symbols() {
        // let input: Vec<_> = INPUT_OTHER_CODE_BLOCK_WITH_SYMBOLS.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = INPUT_OTHER_CODE_BLOCK_WITH_SYMBOLS.lines().map(|x| x.to_owned()).collect();

        let result = process_docs(INPUT_OTHER_CODE_BLOCK_WITH_SYMBOLS, true);

        assert_eq!(result, INPUT_OTHER_CODE_BLOCK_WITH_SYMBOLS);
    }

    const INPUT_INDENT_HEADINGS: &[&str] = &[
        "# heading 1",
        "some text",
        "## heading 2",
        "some other text",
    ];

    const EXPECTED_INDENT_HEADINGS: &[&str] = &[
        "## heading 1",
        "some text",
        "### heading 2",
        "some other text",
    ];

    #[test]
    fn indent_markdown_headings() {
        // let input: Vec<_> = INPUT_INDENT_HEADINGS.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = EXPECTED_INDENT_HEADINGS.lines().collect();

        let result = process_docs(INPUT_INDENT_HEADINGS, true);

        assert_eq!(result, EXPECTED_INDENT_HEADINGS);
    }

    #[test]
    fn do_not_indent_markdown_headings() {
        // let input: Vec<_> = INPUT_INDENT_HEADINGS.lines().map(|x| x.to_owned()).collect();
        // let expected: Vec<_> = INPUT_INDENT_HEADINGS.lines().collect();

        let result = process_docs(INPUT_INDENT_HEADINGS, false);

        assert_eq!(result, INPUT_INDENT_HEADINGS);
    }
}
