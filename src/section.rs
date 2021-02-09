use crate::section::Language::{EN, PL};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

static ARGUMENT_TYPES: [&str; 4] = ["%d", "%s", "%ld", "%%"];

#[derive(Debug, PartialEq)]
pub struct Section<'a> {
    pub label: &'a str,
    pub translations: BTreeMap<Language, &'a str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum Language {
    PL,
    EN,
}

impl Language {
    pub fn variants_count() -> usize {
        2 // PL,EN
    }
}

impl Section<'_> {
    pub fn new() -> Self {
        Section {
            label: "",
            translations: BTreeMap::new(),
        }
    }
    pub fn generate(&self) -> String {
        let mut args: String = "".to_string();
        for x in &self.translations {
            args.push_str(&*format!("\t{lang}\t\"{text}\"\n", lang = x.0, text = x.1));
        }

        format!(
            "section\n\
        \tTXT\t\"{label}\"\n\
        {translations}\
        end",
            label = &self.label,
            translations = args
        )
    }

    pub fn check_translations_arguments(&self) -> bool {
        for x in ARGUMENT_TYPES.iter() {
            let counts = &self
                .translations
                .iter()
                .map(|(_, tr)| tr.match_indices(x).count())
                .collect::<Vec<usize>>();

            if !counts.is_empty() {
                if !counts.iter().all(|f| counts.first() == Some(f)) {
                    return false;
                }
            }
        }

        true
    }
}

pub fn check_string_arguments<T>(left: T, right: T) -> bool
where
    T: AsRef<str>,
{
    for x in ARGUMENT_TYPES.iter() {
        if left.as_ref().match_indices(x).count() != right.as_ref().match_indices(x).count() {
            return false;
        }
    }

    true
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

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::section::Language::{EN, PL};
    use crate::section::{Language, Section};

    #[test]
    fn from_trait_language_works() {
        let pl: Language = "PL".into();
        let en: Language = "EN".into();
        assert_eq!(pl, Language::PL);
        assert_eq!(en, Language::EN);
    }

    #[test]
    fn generates_file_data() {
        let mut section = Section::new();
        section.label = "lab1";
        section.translations.insert(PL, "tr1");
        section.translations.insert(EN, "tr2");

        assert_eq!(
            section.generate(),
            "section\n\tTXT\t\"lab1\"\n\tPL\t\"tr1\"\n\tEN\t\"tr2\"\nend"
        )
    }
}
