#![allow(dead_code)]

#[derive(Debug)]
pub struct Section<'a> {
    pub label: &'a str,
    pub translations: Vec<Translation<'a>>,
}

#[derive(Debug)]
pub struct Translation<'a> {
    pub language: Language,
    pub text: &'a str,
}

#[derive(Clone, Copy, Debug)]
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
