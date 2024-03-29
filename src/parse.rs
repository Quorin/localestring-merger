use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;

use crate::section::Language::{EN, PL};
use crate::section::{Language, Section};
use std::collections::BTreeMap;

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

pub fn omit_line<T>(line: T) -> bool
where
    T: AsRef<str>,
{
    line.as_ref().starts_with("#") || line.as_ref().is_empty()
}

fn extract_text<'a, T: ?Sized>(text: &'a T, key: &'a T) -> Option<&'a str>
where
    T: AsRef<str>,
{
    let elements = text.as_ref().split("\t").collect::<Vec<&str>>();
    if match elements.get(0) {
        Some(v) => v.starts_with(key.as_ref()),
        None => return None,
    } {
        let val = match elements.get(1) {
            Some(a) => a,
            _ => return None,
        }
        .trim_matches(|c| c == '\"' || c == '\\');

        return Some(val);
    }

    None
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid syntax near {0}\t{1}")]
    Syntax(String, String),
    #[error("empty or invalid line near {0}")]
    Empty(String),
    #[error("language {0} already exists in label {1}")]
    LanguageDuplicate(Language, String),
    #[error("label {0} duplicate")]
    LabelDuplicate(String),
    #[error("argument count in label {0} is not equal everywhere")]
    ArgumentMismatch(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn parse_data<'a, T: ?Sized>(data: &'a T) -> Result<Vec<Section<'a>>, ParseError>
where
    T: AsRef<str>,
{
    let mut v: Vec<Section> = vec![];

    for x in data.as_ref().lines().map(|l| l.trim()) {
        if omit_line(x) {
            continue;
        }

        for (keyword, action) in KEYWORDS.iter() {
            if x.starts_with(keyword) {
                if *action == KeywordActions::NewSection {
                    v.push(Section::new());
                    continue;
                }

                let e = extract_text(x, keyword).ok_or(ParseError::Empty(x.to_owned()))?;
                let last = v
                    .last_mut()
                    .ok_or(ParseError::Syntax(keyword.to_string(), e.to_owned()))?;

                match *action {
                    KeywordActions::Label => {
                        last.label = e;
                    }
                    KeywordActions::Translation(lang) => {
                        if last.translations.contains_key(&lang) {
                            return Err(ParseError::LanguageDuplicate(lang, last.label.to_owned()));
                        }
                        if let Some(_) = last.translations.insert(lang, e) {
                            return Err(ParseError::LabelDuplicate(last.label.to_owned()));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(v)
}

pub fn merge_sections<'a>(mut base: Vec<Section<'a>>, new: Vec<Section<'a>>) -> Vec<Section<'a>> {
    for mut x in new {
        let pos = base.iter_mut().find(|f| f.label == x.label);
        if let Some(elem) = pos {
            for new_t in x.translations.iter_mut() {
                if let Some(old_t) = elem.translations.iter_mut().find(|(t, _)| t == &new_t.0) {
                    *old_t.1 = *new_t.1;
                } else {
                    elem.translations.insert(*new_t.0, new_t.1);
                }
            }
        } else {
            base.push(x)
        }
    }

    base
}

pub fn parse_clientside<T>(data: T) -> Result<BTreeMap<String, String>, ParseError>
where
    T: AsRef<str>,
{
    let mut map = BTreeMap::new();

    for x in data.as_ref().lines().map(|l| l.trim()) {
        if omit_line(x) {
            continue;
        }

        let split: Vec<&str> = x.split("\t").collect();
        let (label_slice, translation_slice) = split.split_at(1);
        let label = label_slice.join("");
        if let Some(_) = map.insert(label, translation_slice.join("")) {
            return Err(ParseError::LabelDuplicate(label_slice.join("")));
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::parse::{extract_text, merge_sections, omit_line, parse_data, ParseError};
    use crate::section::Language::{EN, PL};
    use crate::section::Section;

    use super::read_file;

    static FILE_STR: &str = "\
        #hello\n\
        section\n\
            TXT	\"s1\"\n\
            PL	\"pl1\"\n\
            EN	\"en1\"\n\
        end\n
        section\n\
            TXT	\"s2\"\n\
            PL	\"pl2\"\n\
            EN	\"en2\"\n\
        end";

    #[test]
    fn merges_sections_with_different_labels() {
        let mut s1 = Section {
            label: "asd",
            translations: BTreeMap::new(),
        };
        let mut s2 = Section {
            label: "asd2",
            translations: BTreeMap::new(),
        };
        s1.translations.insert(PL, "pl1");
        s2.translations.insert(PL, "pl2");

        let v1 = vec![s1];
        let v2 = vec![s2];

        // result

        let mut res1 = Section {
            label: "asd",
            translations: BTreeMap::new(),
        };
        let mut res2 = Section {
            label: "asd2",
            translations: BTreeMap::new(),
        };

        res1.translations.insert(PL, "pl1");
        res2.translations.insert(PL, "pl2");

        let res_vec = vec![res1, res2];

        assert_eq!(merge_sections(v1, v2), res_vec);
    }

    #[test]
    fn merges_sections_with_duplicated_labels() {
        let mut s1 = Section {
            label: "asd",
            translations: BTreeMap::new(),
        };
        let mut s2 = Section {
            label: "asd",
            translations: BTreeMap::new(),
        };
        s1.translations.insert(PL, "pl1");
        s1.translations.insert(EN, "en1");
        s2.translations.insert(PL, "pl2");

        let v1 = vec![s1];
        let v2 = vec![s2];

        // result

        let mut res1 = Section {
            label: "asd",
            translations: BTreeMap::new(),
        };

        res1.translations.insert(EN, "en1");
        res1.translations.insert(PL, "pl2");

        let res_vec = vec![res1];

        assert_eq!(merge_sections(v1, v2), res_vec);
    }

    #[test]
    fn omit_line_if_empty() {
        assert_eq!(omit_line(""), true);
    }

    #[test]
    fn omit_line_if_commented() {
        assert_eq!(omit_line("#test"), true);
    }

    #[test]
    fn not_omit_line_if_normal() {
        assert_eq!(omit_line("PL\t\"Hello, world!\""), false);
    }

    #[test]
    fn reads_file() {
        assert_eq!(read_file("Cargo.toml").is_ok(), true)
    }

    #[test]
    fn error_if_not_found_file() {
        assert_eq!(read_file("hello.world").is_err(), true)
    }

    #[test]
    fn extract_works() {
        assert_eq!(extract_text("TXT\t\"\\asd\"", "TXT"), Some("asd"))
    }

    #[test]
    fn none_if_no_keyword_while_extract() {
        assert_eq!(extract_text("asd", "TXT"), None)
    }

    #[test]
    fn none_if_no_tab_while_extract() {
        assert_eq!(extract_text("TXT\"\\asd\"", "TXT"), None)
    }

    #[test]
    fn parsing_works() {
        let res = parse_data(FILE_STR);

        let mut sections = vec![Section::new(), Section::new()];
        sections[0].label = "s1";
        sections[1].label = "s2";

        sections[0].translations.insert(PL, "pl1");
        sections[0].translations.insert(EN, "en1");

        sections[1].translations.insert(PL, "pl2");
        sections[1].translations.insert(EN, "en2");

        assert_eq!(res.unwrap(), sections);
    }

    #[test]
    fn error_if_duplicate() {
        let p = parse_data(
            "section\n\
            TXT	\"s1\"\n\
            PL	\"pl1\"\n\
            PL	\"pl2\"\n\
        end",
        );

        assert!(match p {
            Err(ParseError::LanguageDuplicate(PL, label)) => label == "s1".to_owned(),
            _ => false,
        })
    }
}
