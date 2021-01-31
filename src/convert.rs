use crate::section::{Language, Section};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("file is empty")]
    Empty,
    #[error("lines count {0} is not divisible by 2")]
    Syntax(usize),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn convert_data(data: &str, lang: Language) -> Result<Vec<Section>, ConvertError> {
    let lines: Vec<&str> = data
        .lines()
        .map(|l| l.trim().trim_matches(|c| c == '\"' || c == ';'))
        .filter(|l| !l.is_empty() && !l.starts_with("#"))
        .collect();

    if lines.is_empty() {
        return Err(ConvertError::Empty);
    }

    if lines.len() % 2 != 0 {
        return Err(ConvertError::Syntax(lines.len()));
    }

    let mut sections = vec![];
    for line in lines.chunks(2) {
        let mut translations = BTreeMap::new();
        translations.insert(lang, line[1].as_ref());

        sections.push(Section {
            label: line[0],
            translations,
        })
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_data;
    use crate::section::Language::PL;
    use crate::section::Section;

    #[test]
    fn converts() {
        let s = "\
\"hello\";\n\
\"world\";";

        let mut section = Section::new();
        section.label = "hello";
        section.translations.insert(PL, "world");

        let convert = convert_data(s, PL);

        assert!(convert.is_ok());
        assert_eq!(convert.unwrap(), vec![section]);
    }

    #[test]
    fn error_if_not_enough_lines() {
        let s = "\
\"hello\";";

        let convert = convert_data(s, PL);
        assert!(convert.is_err());
    }

    #[test]
    fn error_if_empty() {
        let s = "";

        let convert = convert_data(s, PL);
        assert!(convert.is_err());
    }
}
