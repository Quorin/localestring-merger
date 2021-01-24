use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;

use crate::section::Language::{EN, PL};
use crate::section::{Language, Section, Translation};

#[derive(Debug, PartialEq)]
pub enum KeywordActions {
    NewSection,
    Label,
    Translation(Language),
}

static KEYWORDS: [(&str, KeywordActions); 4] = [
    ("section", KeywordActions::NewSection),
    ("TXT", KeywordActions::Label),
    ("PL", KeywordActions::Translation(PL)),
    ("EN", KeywordActions::Translation(EN)),
];

pub fn read_file<T: AsRef<Path>>(filepath: T) -> std::io::Result<String> {
    read_to_string(filepath)
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
    #[error("invalid syntax near {0}\t{1}")]
    Syntax(&'a str, &'a str),
    #[error("empty or invalid line near {0}")]
    Empty(&'a str),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn parse_data(data: &str) -> Result<Vec<Section>, ParseError> {
    let mut v: Vec<Section> = vec![];

    for x in data.lines().map(|l| l.trim()) {
        if x.starts_with("#") || x.is_empty() {
            continue;
        }

        for (keyword, action) in KEYWORDS.iter() {
            if x.starts_with(keyword) {
                if *action == KeywordActions::NewSection {
                    v.push(Section::new());
                    continue;
                }

                let e = extract_text(x, keyword).ok_or(ParseError::Empty(x))?;
                let last = v.last_mut().ok_or(ParseError::Syntax(keyword, e))?;

                match *action {
                    KeywordActions::Label => {
                        last.label = e;
                    }
                    KeywordActions::Translation(lang) => {
                        last.translations.push(Translation {
                            text: e,
                            language: lang,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(v)
}
