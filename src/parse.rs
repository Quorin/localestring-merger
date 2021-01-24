use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;

use crate::section::Language::{EN, PL};
use crate::section::{Section, Translation};

pub fn read_file<T: AsRef<Path>>(filepath: T) -> String {
    read_to_string(filepath).unwrap()
}

fn extract_text<'a>(text: &'a str, key: &'a str) -> Option<&'a str> {
    let elements = text.split("\t").collect::<Vec<&str>>();
    if elements.get(0).unwrap().starts_with(key) {
        let val = elements
            .get(1)
            .unwrap()
            .trim_matches(|c| c == '\"' || c == '\\');

        return Some(val);
    }

    None
}

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error("Invalid syntax near {0}\t{1}")]
    Syntax(&'a str, &'a str),
}

pub fn parse_data(data: &str) -> Result<Vec<Section>, ParseError> {
    let mut v: Vec<Section> = vec![];

    for x in data.lines().map(|l| l.trim()) {
        if x.starts_with("#") || x.is_empty() {
            continue;
        }

        if x.starts_with("section") {
            v.push(Section::new())
        } else if x.starts_with("TXT") {
            let extracted = extract_text(x, "TXT");
            if let Some(value) = extracted {
                if let Some(last) = v.last_mut() {
                    last.label = value;
                } else {
                    return Err(ParseError::Syntax("TXT", value));
                }
            }
        } else if x.starts_with("PL") {
            let extracted = extract_text(x, "PL");
            if let Some(value) = extracted {
                if let Some(last) = v.last_mut() {
                    last.translations.push(Translation {
                        text: value,
                        language: PL,
                    })
                } else {
                    return Err(ParseError::Syntax("PL", value));
                }
            }
        } else if x.starts_with("EN") {
            let extracted = extract_text(x, "EN");
            if let Some(value) = extracted {
                if let Some(last) = v.last_mut() {
                    last.translations.push(Translation {
                        text: value,
                        language: EN,
                    })
                } else {
                    return Err(ParseError::Syntax("EN", value));
                }
            }
        }
    }

    Ok(v)
}
