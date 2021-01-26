use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;

use crate::section::Language::{EN, PL};
use crate::section::{Language, Section};

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

pub fn omit_line(line: &str) -> bool {
    line.starts_with("#") || line.is_empty()
}

fn extract_text<'a>(text: &'a str, key: &'a str) -> Option<&'a str> {
    let elements = text.split("\t").collect::<Vec<&str>>();
    if match elements.get(0) {
        Some(v) => v.starts_with(key),
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
pub enum ParseError<'a> {
    #[error("invalid syntax near {0}\t{1}")]
    Syntax(&'a str, &'a str),
    #[error("empty or invalid line near {0}")]
    Empty(&'a str),
    #[error("language {0} already exists in label {1}")]
    LanguageDuplicate(Language, &'a str),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn parse_data(data: &str) -> Result<Vec<Section>, ParseError> {
    let mut v: Vec<Section> = vec![];

    for x in data.lines().map(|l| l.trim()) {
        if omit_line(x) {
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
                        if last.translations.contains_key(&lang) {
                            return Err(ParseError::LanguageDuplicate(lang, last.label));
                        }
                        last.translations.insert(lang, e);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(v)
}

#[cfg(test)]
mod tests {
    use crate::parse::{extract_text, omit_line, parse_data, ParseError};
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
        assert!(match parse_data(
            "section\n\
            TXT	\"s1\"\n\
            PL	\"pl1\"\n\
            PL	\"pl2\"\n\
        end"
        ) {
            Err(ParseError::LanguageDuplicate(PL, "s1")) => true,
            _ => false,
        })
    }
}
