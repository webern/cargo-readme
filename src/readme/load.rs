//! Load raw doc comments

use std::io::{Read, BufRead, BufReader};

use super::DocStyle;

const ERR_MIX_STYLES: &str = "Cannot mix singleline and multiline doc comments";

pub fn load_docs<R: Read>(reader: R) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let reader = BufReader::new(reader);

    let mut style = DocStyle::NoDoc;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("{}", e))?;

        if style == DocStyle::NoDoc {
            if line.starts_with("//!") {
                style = DocStyle::SingleLine;
                result.push(line);
            } else if line.starts_with("/*!") {
                style = DocStyle::MultiLine;
                result.push(line);
            }
        } else if style == DocStyle::SingleLine {
            if line.starts_with("//!") {
                result.push(line);
            } else if line.starts_with("/*!") {
                return Err(ERR_MIX_STYLES.to_owned());
            } else if line.trim().len() > 0 {
                // doc ends, code starts
                break;
            }
        } else if style == DocStyle::MultiLine {
            // cannot call `ends_with` on `line` after moving it into `result`
            let end_doc_comment = line.ends_with("*/");
            result.push(line);
            if end_doc_comment {
                break;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    const INPUT_SINGLELINE: &str = r#"use std::io::prelude::*;
//! first line
//!
//! ```
//! let rust_code = "safe";
//! ```
//!
//! ```C
//! int i = 0; // no rust code
//! ```
use std::any::Any;

fn main() {}"#;

    const INPUT_MULTILINE: &str = r#"use std::io::prelude::*;
/*!
first line

```
let rust_code = "safe";
```

```C
int i = 0; // no rust code
```
*/
use std::any::Any;

fn main() {}"#;

    const INPUT_MIXED: &str = r#"//! first line
/*! start multiline
end multiline */"#;

    const EXPECTED_SINGLELINE: &str = r#"//! first line
//!
//! ```
//! let rust_code = "safe";
//! ```
//!
//! ```C
//! int i = 0; // no rust code
//! ```"#;

    const EXPECTED_MULTILINE: &str = r#"/*!
first line

```
let rust_code = "safe";
```

```C
int i = 0; // no rust code
```
*/"#;

    #[test]
    fn single_line() {
        let expected: Vec<_> = EXPECTED_SINGLELINE.lines().collect();
        let result = load_docs(Cursor::new(INPUT_SINGLELINE.as_bytes())).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn multi_line() {
        let expected: Vec<_> = EXPECTED_MULTILINE.lines().collect();
        let result = load_docs(Cursor::new(INPUT_MULTILINE.as_bytes())).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot mix singleline and multiline doc comments")]
    fn mix_styles() {
        load_docs(Cursor::new(INPUT_MIXED.as_bytes())).unwrap();
    }
}
