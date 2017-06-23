//! Extract the raw doc comments from rust source code

use std::io::{Read, BufRead, BufReader};

const ERR_MIX_STYLES: &str = "Cannot mix singleline and multiline doc comments";

pub fn extract_docs<R: Read>(reader: R) -> Result<Vec<String>, String> {
    let mut reader = BufReader::new(reader);

    let mut line = String::new();
    let mut bytes_read = reader.read_line(&mut line).map_err(|e| format!("{}", e))?;

    while bytes_read > 0 {
        if line.starts_with("//!") {
            return extract_docs_singleline_style(line, reader);
        }
        if line.starts_with("/*!") {
            return extract_docs_multiline_style(line, reader);
        }

        line.clear();
        bytes_read = reader.read_line(&mut line).map_err(|e| format!("{}", e))?;
    }

    Ok(Vec::new())
}

fn extract_docs_singleline_style<R: Read>(first_line: String, reader: BufReader<R>) -> Result<Vec<String>, String> {
    let mut result = vec![normalize_line(first_line)];

    for line in reader.lines() {
        let line = line.map_err(|e| format!("{}", e))?;

        if line.starts_with("//!") {
            result.push(normalize_line(line));
        } else if line.starts_with("/*!") {
            return Err(ERR_MIX_STYLES.to_owned());
        } else if line.trim().len() > 0 {
            // doc ends, code starts
            break;
        }
    }

    Ok(result)
}

fn extract_docs_multiline_style<R: Read>(first_line: String, reader: BufReader<R>) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    if first_line.starts_with("/*!") && first_line.trim().len() > "/*!".len() {
        result.push(normalize_line(first_line));
    }

    for line in reader.lines() {
        let line = line.map_err(|e| format!("{}", e))?;

        if let Some(pos) = line.rfind("*/") {
            let mut line = line;
            line.split_off(pos);
            if !line.trim().is_empty() {
                result.push(line);
            }
            break
        }

        result.push(line.trim_right().to_owned());
    }

    Ok(result)
}

/// Strip the "//!" or "/*!" from it and a single whitespace
fn normalize_line(mut line: String) -> String {
    if line.trim() == "//!" || line.trim() == "/*!" {
        line.clear();
        line
    } else {
        // if the first character after the comment mark is " ", remove it
        let split_at = if line.find(" ") == Some(3) { 4 } else { 3 };
        line.split_at(split_at).1.trim_right().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    const EXPECTED: &[&str] = &[
        "first line",
        "",
        "```",
        "let rust_code = \"safe\";",
        "```",
        "",
        "```C",
        "int i = 0; // no rust code",
        "```",
    ];

    const INPUT_SINGLELINE: &str = "\
//! first line
//!
//! ```
//! let rust_code = \"safe\";
//! ```
//!
//! ```C
//! int i = 0; // no rust code
//! ```
use std::any::Any;

fn main() {

}
";

    #[test]
    fn extract_docs_singleline_style() {
        let reader = Cursor::new(INPUT_SINGLELINE.as_bytes());
        let result = extract_docs(reader).unwrap();
        assert_eq!(result, EXPECTED);
    }

    const INPUT_MULTILINE: &str = "\
/*!
first line

```
let rust_code = \"safe\";
```

```C
int i = 0; // no rust code
```
*/
use std::any::Any;

fn main() {

}
";

    #[test]
    fn extract_docs_multiline_style() {
        let reader = Cursor::new(INPUT_MULTILINE.as_bytes());
        let result = extract_docs(reader).unwrap();
        assert_eq!(result, EXPECTED);
    }

    const INPUT_MIXED: &str = "\
//! single line
/*!
start multiline
end multiline
*/";

    #[test]
    fn extract_docs_mix_styles_should_fail() {
        let reader = Cursor::new(INPUT_MIXED.as_bytes());
        let result = extract_docs(reader);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot mix singleline and multiline doc comments");
    }
}
