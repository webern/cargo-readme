//! Transform code blocks from rustdoc into markdown
//!
//! Rewrite code block start tags, changing rustdoc into equivalent in markdown:
//! - code blocks that rustdoc treats as rust are converted to "```rust"
//! - "```text" has its language stripped, becoming a plain "```"
//! - markdown heading are indentend to be one level lower, so the crate name is at the top level

use regex::Regex;
use std::{
    iter::{IntoIterator, Iterator},
    sync::LazyLock,
};

// A fenced code block opening: 3-4 backticks/tildes and an optional info string.
static RE_CODE_FENCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<delimiter>`{3,4}|~{3,4})(?P<info>.*)$").unwrap());

/// Does this code block info string denote rust?
///
/// Mirrors rustdoc's `LangString::parse` (an unpublished, `rustc_private`
/// function we cannot depend on directly), ported from rust-lang/rust@a9cf06b5.
/// A block is rust unless a `custom` tag appears, or an unrecognized tag appears
/// without an explicit `rust` tag. To check for drift, diff `LangString::parse`
/// against this function.
///
/// Typo variants like `should-panic` or `norun` are deliberately not rust:
/// rustdoc treats them as markers for other languages, so we keep them verbatim.
fn is_rust_code(info: &str) -> bool {
    let mut seen_rust_tags = false;
    let mut seen_other_tags = false;
    let mut seen_custom_tag = false;

    for token in info.split([' ', ',', '\t']).filter(|t| !t.is_empty()) {
        match token {
            "should_panic" | "no_run" | "ignore" => {
                seen_rust_tags = !seen_other_tags;
            }
            "rust" => {
                seen_rust_tags = true;
            }
            "custom" => {
                seen_custom_tag = true;
            }
            "test_harness" | "compile_fail" | "standalone_crate" => {
                seen_rust_tags = !seen_other_tags || seen_rust_tags;
            }
            _ if token.starts_with("ignore-") => {
                seen_rust_tags = !seen_other_tags;
            }
            // `edition*` sets the edition only; it is neither a rust nor an other tag.
            _ if token.starts_with("edition") => {}
            _ if is_error_code(token) => {
                seen_rust_tags = !seen_other_tags || seen_rust_tags;
            }
            _ => {
                seen_other_tags = true;
            }
        }
    }

    !seen_custom_tag && (!seen_other_tags || seen_rust_tags)
}

/// A rustdoc error-code tag: `E` followed by exactly four ASCII digits (e.g. `E0277`).
fn is_error_code(token: &str) -> bool {
    matches!(token.strip_prefix('E'), Some(rest) if rest.len() == 4 && rest.bytes().all(|b| b.is_ascii_digit()))
}

/// Process and concatenate the doc lines into a single String
///
/// The processing transforms doc tests into regular rust code blocks and optionally indent the
/// markdown headings in order to leave the top heading to the crate name
pub fn process_docs<S: Into<String>, L: Into<Vec<S>>>(
    lines: L,
    indent_headings: bool,
) -> Vec<String> {
    lines.into().into_iter().process_docs(indent_headings)
}

pub struct Processor {
    section: Section,
    indent_headings: bool,
    delimiter: Option<String>,
}

impl Processor {
    pub fn new(indent_headings: bool) -> Self {
        Processor {
            section: Section::None,
            indent_headings,
            delimiter: None,
        }
    }

    pub fn process_line(&mut self, mut line: String) -> Option<String> {
        // Skip lines that should be hidden in docs
        if self.section == Section::CodeRust && line.trim_ascii_start().starts_with("# ") {
            return None;
        }

        // indent heading when outside code
        if self.indent_headings && self.section == Section::None && line.starts_with("#") {
            line.insert(0, '#');
        } else if self.section == Section::None {
            let l = line.clone();
            if let Some(cap) = RE_CODE_FENCE.captures(&l) {
                let delimiter = cap.name("delimiter").unwrap().as_str();
                let info = cap.name("info").unwrap().as_str();
                self.delimiter = Some(delimiter.to_owned());
                if is_rust_code(info) {
                    self.section = Section::CodeRust;
                    line = format!("{delimiter}rust");
                } else {
                    self.section = Section::CodeOther;
                    // "```text" is stripped to a plain fence; other languages are kept as-is.
                    if info.trim() == "text" {
                        line = delimiter.to_owned();
                    }
                }
            }
        } else if self.section != Section::None && Some(&line) == self.delimiter.as_ref() {
            self.section = Section::None;
            line = self.delimiter.take().unwrap_or("```".to_owned());
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

    const EXPECTED_HIDDEN_LINE: &[&str] =
        &["```rust", "#[visible]", "let visible = \"visible\";", "```"];

    #[test]
    fn hide_line_in_rust_code_block() {
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
        "```compile_fail",
        "x y z",
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
        "```rust",
        "x y z",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    #[test]
    fn transform_rust_code_block() {
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
        "```rust,compile_fail",
        "x y z",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    #[test]
    fn transform_rust_code_block_with_prefix() {
        let result = process_docs(INPUT_RUST_CODE_BLOCK_RUST_PREFIX, true);
        assert_eq!(result, EXPECTED_RUST_CODE_BLOCK);
    }

    const INPUT_RUST_CODE_BLOCK_ALL_ANNOTATIONS: &[&str] = &[
        "```test_harness",
        "let a = test_harness;",
        "```",
        "",
        "```standalone_crate",
        "let a = standalone;",
        "```",
        "",
        "```edition2018",
        "let a = edition;",
        "```",
        "",
        "```E0277",
        "let a = error_code;",
        "```",
        "",
        "```ignore-windows",
        "let a = ignore_target;",
        "```",
    ];

    const EXPECTED_RUST_CODE_BLOCK_ALL_ANNOTATIONS: &[&str] = &[
        "```rust",
        "let a = test_harness;",
        "```",
        "",
        "```rust",
        "let a = standalone;",
        "```",
        "",
        "```rust",
        "let a = edition;",
        "```",
        "",
        "```rust",
        "let a = error_code;",
        "```",
        "",
        "```rust",
        "let a = ignore_target;",
        "```",
    ];

    #[test]
    fn transform_rust_code_block_all_annotations() {
        let result = process_docs(INPUT_RUST_CODE_BLOCK_ALL_ANNOTATIONS, true);
        assert_eq!(result, EXPECTED_RUST_CODE_BLOCK_ALL_ANNOTATIONS);
    }

    const INPUT_RUST_CODE_BLOCK_MULTIPLE_TAGS: &[&str] = &[
        "```rust,no_run",
        "let a = 1;",
        "```",
        "",
        "```no_run,should_panic",
        "let a = 2;",
        "```",
        "",
        "```rust,edition2021,compile_fail",
        "let a = 3;",
        "```",
    ];

    const EXPECTED_RUST_CODE_BLOCK_MULTIPLE_TAGS: &[&str] = &[
        "```rust",
        "let a = 1;",
        "```",
        "",
        "```rust",
        "let a = 2;",
        "```",
        "",
        "```rust",
        "let a = 3;",
        "```",
    ];

    #[test]
    fn transform_rust_code_block_multiple_tags() {
        let result = process_docs(INPUT_RUST_CODE_BLOCK_MULTIPLE_TAGS, true);
        assert_eq!(result, EXPECTED_RUST_CODE_BLOCK_MULTIPLE_TAGS);
    }

    // rustdoc treats typo variants (hyphenated / run-together) as markers for
    // *other* languages, emitting a "did you mean" lint rather than compiling
    // them as Rust. We faithfully keep them verbatim as non-Rust blocks.
    const INPUT_TYPO_ANNOTATIONS_NOT_RUST: &[&str] = &[
        "```should-panic",
        "let a = 1;",
        "```",
        "",
        "```norun",
        "let a = 2;",
        "```",
        "",
        "```compile-fail",
        "let a = 3;",
        "```",
    ];

    #[test]
    fn typo_annotations_are_not_rust() {
        let result = process_docs(INPUT_TYPO_ANNOTATIONS_NOT_RUST, true);
        assert_eq!(result, INPUT_TYPO_ANNOTATIONS_NOT_RUST);
    }

    const INPUT_TEXT_BLOCK: &[&str] = &["```text", "this is text", "```"];

    const EXPECTED_TEXT_BLOCK: &[&str] = &["```", "this is text", "```"];

    #[test]
    fn transform_text_block() {
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
        let result = process_docs(INPUT_INDENT_HEADINGS, true);
        assert_eq!(result, EXPECTED_INDENT_HEADINGS);
    }

    #[test]
    fn do_not_indent_markdown_headings() {
        let result = process_docs(INPUT_INDENT_HEADINGS, false);
        assert_eq!(result, INPUT_INDENT_HEADINGS);
    }

    const INPUT_ALTERNATE_DELIMITER_4_BACKTICKS: &[&str] = &["````", "let i = 1;", "````"];

    const EXPECTED_ALTERNATE_DELIMITER_4_BACKTICKS: &[&str] = &["````rust", "let i = 1;", "````"];

    #[test]
    fn alternate_delimiter_4_backticks() {
        let result = process_docs(INPUT_ALTERNATE_DELIMITER_4_BACKTICKS, false);
        assert_eq!(result, EXPECTED_ALTERNATE_DELIMITER_4_BACKTICKS);
    }

    const INPUT_ALTERNATE_DELIMITER_4_BACKTICKS_NESTED: &[&str] = &[
        "````",
        "//! ```",
        "//! let i = 1;",
        "//! ```",
        "```python",
        "i = 1",
        "```",
        "````",
    ];

    const EXPECTED_ALTERNATE_DELIMITER_4_BACKTICKS_NESTED: &[&str] = &[
        "````rust",
        "//! ```",
        "//! let i = 1;",
        "//! ```",
        "```python",
        "i = 1",
        "```",
        "````",
    ];

    #[test]
    fn alternate_delimiter_4_backticks_nested() {
        let result = process_docs(INPUT_ALTERNATE_DELIMITER_4_BACKTICKS_NESTED, false);
        assert_eq!(result, EXPECTED_ALTERNATE_DELIMITER_4_BACKTICKS_NESTED);
    }

    const INPUT_ALTERNATE_DELIMITER_3_TILDES: &[&str] = &["~~~", "let i = 1;", "~~~"];

    const EXPECTED_ALTERNATE_DELIMITER_3_TILDES: &[&str] = &["~~~rust", "let i = 1;", "~~~"];

    #[test]
    fn alternate_delimiter_3_tildes() {
        let result = process_docs(INPUT_ALTERNATE_DELIMITER_3_TILDES, false);
        assert_eq!(result, EXPECTED_ALTERNATE_DELIMITER_3_TILDES);
    }

    const INPUT_ALTERNATE_DELIMITER_4_TILDES: &[&str] = &["~~~~", "let i = 1;", "~~~~"];

    const EXPECTED_ALTERNATE_DELIMITER_4_TILDES: &[&str] = &["~~~~rust", "let i = 1;", "~~~~"];

    #[test]
    fn alternate_delimiter_4_tildes() {
        let result = process_docs(INPUT_ALTERNATE_DELIMITER_4_TILDES, false);
        assert_eq!(result, EXPECTED_ALTERNATE_DELIMITER_4_TILDES);
    }

    const INPUT_ALTERNATE_DELIMITER_MIXED: &[&str] = &[
        "```",
        "let i = 1;",
        "```",
        "````",
        "//! ```",
        "//! let i = 1;",
        "//! ```",
        "```python",
        "i = 1",
        "```",
        "````",
        "~~~markdown",
        "```python",
        "i = 1",
        "```",
        "~~~",
    ];

    const EXPECTED_ALTERNATE_DELIMITER_MIXED: &[&str] = &[
        "```rust",
        "let i = 1;",
        "```",
        "````rust",
        "//! ```",
        "//! let i = 1;",
        "//! ```",
        "```python",
        "i = 1",
        "```",
        "````",
        "~~~markdown",
        "```python",
        "i = 1",
        "```",
        "~~~",
    ];

    #[test]
    fn alternate_delimiter_mixed() {
        let result = process_docs(INPUT_ALTERNATE_DELIMITER_MIXED, false);
        assert_eq!(result, EXPECTED_ALTERNATE_DELIMITER_MIXED);
    }
}
