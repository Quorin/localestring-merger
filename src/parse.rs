use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;

use crate::section::Language::{EN, PL};
use crate::section::{Language, Section, Translation};

#[derive(Debug)]
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
    #[error("invalid syntax near {0}\t{1}")]
    Syntax(&'a str, &'a str),
    #[error("empty or invalid line near {0}")]
    Empty(&'a str),
}

pub fn parse_data(data: &str) -> Result<Vec<Section>, ParseError> {
    let mut v: Vec<Section> = vec![];

    for x in data.lines().map(|l| l.trim()) {
        if x.starts_with("#") || x.is_empty() {
            continue;
        }

        for (keyword, action) in KEYWORDS.iter() {
            if x.starts_with(keyword) {
                match *action {
                    KeywordActions::NewSection => v.push(Section::new()),
                    _ => {
                        let e = extract_text(x, keyword);
                        if e.is_none() {
                            return Err(ParseError::Empty(x));
                        }

                        let last = v.last_mut();
                        if last.is_none() {
                            return Err(ParseError::Syntax(keyword, e.unwrap()));
                        }

                        match *action {
                            KeywordActions::Label => {
                                last.unwrap().label = e.unwrap();
                            }
                            KeywordActions::Translation(lang) => {
                                last.unwrap().translations.push(Translation {
                                    text: e.unwrap(),
                                    language: lang,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(v)
}
