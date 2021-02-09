use crate::parse::{parse_clientside, ParseError};
use crate::section::{Language, Section};

pub fn find_incomplete_sections(sections: Vec<Section>) -> Vec<&str> {
    let mut unfinished_translations = vec![];

    for s in sections.iter() {
        if s.translations.len() != Language::variants_count() {
            unfinished_translations.push(s.label);
        }
    }

    unfinished_translations
}

pub fn find_missing_labels<'a, T>(data: T, second_data: T) -> Result<Vec<String>, ParseError>
where
    T: AsRef<str>,
{
    let first_map = parse_clientside(data)?;
    let second_map = parse_clientside(second_data)?;
    let mut missing_vec: Vec<String> = vec![];

    for (x, _) in first_map.iter() {
        if !second_map.contains_key(x) {
            missing_vec.push(x.to_owned());
        }
    }

    Ok(missing_vec)
}

#[cfg(test)]
mod tests {
    use crate::find::find_incomplete_sections;
    use crate::section::Language::{EN, PL};
    use crate::section::Section;

    #[test]
    fn finds_incomplete_translations() {
        let mut s1 = Section::new();
        s1.label = "test";
        s1.translations.insert(PL, "asd");

        assert_eq!(find_incomplete_sections(vec![s1]), vec!["test"])
    }

    #[test]
    fn not_return_complete_sections() {
        let mut s1 = Section::new();
        s1.label = "test";
        s1.translations.insert(PL, "asd");
        s1.translations.insert(EN, "asd");

        let empty: Vec<&str> = vec![];

        assert_eq!(find_incomplete_sections(vec![s1]), empty)
    }
}
