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
