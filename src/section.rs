use crate::section::Language::{EN, PL};

#[derive(Debug, PartialEq)]
pub struct Section<'a> {
    pub label: &'a str,
    pub translations: Vec<Translation<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Translation<'a> {
    pub language: Language,
    pub text: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Language {
    PL,
    EN,
}

impl Section<'_> {
    pub fn new() -> Self {
        Section {
            label: "",
            translations: vec![],
        }
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "PL" => PL,
            "EN" => EN,
            lang => panic!("invalid language {}", lang),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::section::Language;

    #[test]
    fn from_trait_language_works() {
        let pl: Language = "PL".into();
        let en: Language = "EN".into();
        assert_eq!(pl, Language::PL);
        assert_eq!(en, Language::EN);
    }
}
