use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

pub fn read(source: &mut File) -> Vec<String> {
    let reader = BufReader::new(source);
    let mut in_code = false;

    reader.lines().filter_map(|line| {
        let line = line.unwrap();
        if line.starts_with("//!") {
            if line.starts_with("//! ```") {
                in_code = !in_code;
            }
            else if line.starts_with("//! # ") && in_code {
                return None;
            }

            // Remove leading '//!' before returning the line
            if line.len() == 3 {
                Some("".to_owned())
            }
            else {
                Some(line[4..].to_owned())
            }
        } else {
            return None
        }
    })
    .collect()
}

pub fn write(dest: &mut File, data: &Vec<String>) -> io::Result<()> {
    let mut writer = BufWriter::new(dest);

    for line in data {
        try!(writeln!(writer, "{}", line));
    }

    Ok(())
}
