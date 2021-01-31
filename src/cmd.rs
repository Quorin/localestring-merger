use crate::convert::{convert_data, ConvertError};
use crate::find::find_occurrences;
use crate::parse::{merge_sections, parse_data, read_file, ParseError};
use crate::section::Language;
use crate::section::Language::{EN, PL};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::fmt::{Display, Formatter};
use std::path::Path;

pub enum Action {
    Merge,
    Convert,
    FindIncomplete,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Merge => write!(f, "{}", "Merge translations"),
            Action::Convert => write!(f, "{}", "Convert old file"),
            Action::FindIncomplete => write!(f, "{}", "Find incomplete translations"),
        }
    }
}

impl From<usize> for Action {
    fn from(v: usize) -> Self {
        match v {
            0 => Action::Merge,
            1 => Action::Convert,
            2 => Action::FindIncomplete,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Language {
    fn from(l: usize) -> Self {
        match l {
            0 => PL,
            1 => EN,
            _ => unreachable!(),
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let select_items = vec![Action::Merge, Action::Convert, Action::FindIncomplete];
    let theme = &ColorfulTheme::default();
    let option: Action = Select::with_theme(theme)
        .with_prompt("Choose action:")
        .items(&select_items)
        .default(0)
        .interact()?
        .into();

    match option {
        Action::Merge => {
            let cur_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename containing current translations")
                .default("locale_string.txt".into())
                .interact_text()?;

            let newer_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename containing newer translations")
                .default("locale_string2.txt".into())
                .interact_text()?;

            let save_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename to which new translations will be saved")
                .default("locale_string_new.txt".into())
                .interact_text()?;

            merge(&cur_file, &newer_file, &save_file).unwrap();
        }
        Action::Convert => {
            let old_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename containing old translations")
                .default("locale_string_old.txt".into())
                .interact_text()?;

            let languages = vec![PL, EN];
            let lang: Language = Select::with_theme(theme)
                .with_prompt("What language is in the file?")
                .items(&languages)
                .default(0)
                .interact()?
                .into();

            let new_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename to which converted translations will be saved")
                .default("locale_string_new.txt".into())
                .interact_text()?;

            convert(&old_file, &new_file, lang).unwrap();
        }
        Action::FindIncomplete => {
            let file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename containing translations")
                .default("locale_string.txt".into())
                .interact_text()?;

            let save_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename to which incomplete translations will be saved")
                .default("locale_string_incomplete.txt".into())
                .interact_text()?;

            find_incomplete(&file, &save_file).unwrap();
        }
    }

    Ok(())
}

fn find_incomplete<T>(file: T, save_file: T) -> Result<(), ParseError>
where
    T: AsRef<Path>,
{
    let data = &*read_file(file)?;
    let parsed_data = parse_data(data)?;
    let occurrences: String = find_occurrences(parsed_data)
        .iter()
        .map(|s| format!("{}\n", *s))
        .collect();

    let _ = std::fs::write(save_file, &occurrences)?;

    Ok(())
}

fn convert<T>(old_file: T, save_file: T, lang: Language) -> Result<(), ConvertError>
where
    T: AsRef<Path>,
{
    let old_data = &*read_file(old_file)?;
    let converted_data = convert_data(old_data, lang)?;
    let generated: String = converted_data
        .iter()
        .map(|f| format!("{}\n\n", f.generate()))
        .collect();

    let _ = std::fs::write(save_file, &generated)?;

    Ok(())
}

fn merge<T>(cur_file: T, new_file: T, save_file: T) -> Result<(), ParseError>
where
    T: AsRef<Path>,
{
    let cur_data = &*read_file(cur_file)?;
    let new_data = &*read_file(new_file)?;

    let cur_sections = parse_data(cur_data)?;
    let new_sections = parse_data(new_data)?;
    let merged = merge_sections(cur_sections, new_sections);
    let generated: String = merged
        .iter()
        .map(|f| format!("{}\n\n", f.generate()))
        .collect();

    let _ = std::fs::write(save_file, &generated)?;

    Ok(())
}
