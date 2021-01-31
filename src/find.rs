use crate::section::{Language, Section};

pub fn find_occurrences(sections: Vec<Section>) -> Vec<&str> {
    let mut unfinished_translations = vec![];

    for s in sections.iter() {
        if s.translations.len() != Language::variants_count() {
            unfinished_translations.push(s.label);
        }
    }

    unfinished_translations
}
