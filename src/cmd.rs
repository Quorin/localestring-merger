use crate::convert::{convert_data, ConvertError};
use crate::find::{find_incomplete_sections, find_missing_labels};
use crate::parse::ParseError::ArgumentMismatch;
use crate::parse::{merge_sections, parse_clientside, parse_data, read_file, ParseError};
use crate::section::Language::{EN, PL};
use crate::section::{check_string_arguments, Language};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
pub enum LocaleType {
    LocaleString,
    // serverside
    LocaleGameInterface, // clientside
}

impl Display for LocaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocaleType::LocaleString => write!(f, "locale_string (server-side)"),
            LocaleType::LocaleGameInterface => {
                write!(f, "locale_game/locale_interface (client-side)")
            }
        }
    }
}

impl From<usize> for LocaleType {
    fn from(v: usize) -> Self {
        match v {
            0 => LocaleType::LocaleString,
            1 => LocaleType::LocaleGameInterface,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
pub enum Action {
    Merge,
    Convert,
    FindIncomplete,
    CheckArguments,
    CheckTranslationsDiversity,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Merge => write!(f, "Merge translations"),
            Action::Convert => write!(f, "Convert old file"),
            Action::FindIncomplete => write!(f, "Find incomplete translations"),
            Action::CheckArguments => write!(f, "Check arguments"),
            Action::CheckTranslationsDiversity => write!(f, "Check translations diversity"),
        }
    }
}

impl From<usize> for Action {
    fn from(v: usize) -> Self {
        match v {
            0 => Action::Merge,
            1 => Action::Convert,
            2 => Action::FindIncomplete,
            3 => Action::CheckArguments,
            4 => Action::CheckTranslationsDiversity,
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
    let select_items = vec![
        Action::Merge,
        Action::Convert,
        Action::FindIncomplete,
        Action::CheckArguments,
        Action::CheckTranslationsDiversity,
    ];
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

            if let Err(e) = merge(&cur_file, &newer_file, &save_file) {
                println!("Error: {:#?}", e);
            }
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

            if let Err(e) = convert(&old_file, &new_file, lang) {
                println!("Error: {:#?}", e);
            }
        }
        Action::FindIncomplete => {
            let locale_types = vec![LocaleType::LocaleString, LocaleType::LocaleGameInterface];
            let selected_locale_type: LocaleType = Select::with_theme(theme)
                .with_prompt("Choose file type:")
                .items(&locale_types)
                .default(0)
                .interact()?
                .into();

            let file: Option<String>;
            let mut second_file: Option<String> = None;

            if selected_locale_type == LocaleType::LocaleGameInterface {
                file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations")
                        .default("locale_game.txt".into())
                        .interact_text()?,
                );
                second_file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations to compare")
                        .default("locale_game2.txt".into())
                        .interact_text()?,
                );
            } else {
                file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations")
                        .default("locale_string.txt".into())
                        .interact_text()?,
                );
            }

            let save_file: String = Input::with_theme(theme)
                .with_prompt("Enter the filename to which incomplete translations will be saved")
                .default("locale_string_incomplete.txt".into())
                .interact_text()?;

            if let Err(e) = find_incomplete(
                file.as_ref(),
                second_file.as_ref(),
                &save_file,
                selected_locale_type,
            ) {
                println!("Error: {:#?}", e);
            }
        }
        Action::CheckArguments | Action::CheckTranslationsDiversity => {
            let locale_types = vec![LocaleType::LocaleString, LocaleType::LocaleGameInterface];
            let selected_locale_type: LocaleType = Select::with_theme(theme)
                .with_prompt("Choose file type:")
                .items(&locale_types)
                .default(0)
                .interact()?
                .into();

            let file: Option<String>;
            let mut second_file: Option<String> = None;

            if selected_locale_type == LocaleType::LocaleGameInterface {
                file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations")
                        .default("locale_game.txt".into())
                        .interact_text()?,
                );
                second_file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations to compare")
                        .default("locale_game2.txt".into())
                        .interact_text()?,
                );
            } else {
                file = Some(
                    Input::with_theme(theme)
                        .with_prompt("Enter the filename containing translations")
                        .default("locale_string.txt".into())
                        .interact_text()?,
                );
            }

            if option == Action::CheckTranslationsDiversity {
                let save_file: String = Input::with_theme(theme)
                    .with_prompt(
                        "Enter the filename to which incomplete translations will be saved",
                    )
                    .default("locale_string_no_diversity.txt".into())
                    .interact_text()?;

                if let Err(e) = check_diversity(
                    file.as_ref(),
                    second_file.as_ref(),
                    &save_file,
                    selected_locale_type,
                ) {
                    println!("Error: {:#?}", e);
                }
            } else {
                if let Err(e) =
                    check_arguments(file.as_ref(), second_file.as_ref(), selected_locale_type)
                {
                    println!("Error: {:#?}", e);
                }
            }
        }
    }

    Ok(())
}

fn check_diversity<T>(
    file: Option<T>,
    secondary_file: Option<T>,
    save_file: T,
    locale_type: LocaleType,
) -> Result<(), ParseError>
where
    T: AsRef<Path>,
{
    let first_file_data = &*read_file(file.unwrap())?;
    match locale_type {
        LocaleType::LocaleString => {
            let sections = parse_data(first_file_data)?;
            let missing_diversity_sections: String = sections
                .iter()
                .filter(|s| !s.check_translations_diversity())
                .map(|s| format!("{}\n", s.label))
                .collect();

            let _ = std::fs::write(save_file, &missing_diversity_sections)?;
        }
        LocaleType::LocaleGameInterface => {
            let second_file_data = &*read_file(secondary_file.unwrap())?;

            let map_first = parse_clientside(first_file_data)?;
            let map_second = parse_clientside(second_file_data)?;

            let mut diversity_vec = vec![];

            for (k, v) in map_first.iter() {
                if let Some(map_value) = map_second.get(k) {
                    if map_value == v {
                        diversity_vec.push(v);
                    }
                }
            }

            let _ = std::fs::write(
                save_file,
                &diversity_vec
                    .iter()
                    .map(|s| format!("{}\n", *s))
                    .collect::<String>(),
            )?;
        }
    }

    Ok(())
}

fn check_arguments<T>(
    file: Option<T>,
    secondary_file: Option<T>,
    locale_type: LocaleType,
) -> Result<(), ParseError>
where
    T: AsRef<Path>,
{
    let first_file_data = &*read_file(file.unwrap())?;
    match locale_type {
        LocaleType::LocaleString => {
            let sections = parse_data(first_file_data)?;
            for s in &sections {
                if !s.check_translations_arguments() {
                    return Err(ParseError::ArgumentMismatch(s.label.to_string()));
                }
            }
        }
        LocaleType::LocaleGameInterface => {
            let second_file_data = &*read_file(secondary_file.unwrap())?;

            let map_first = parse_clientside(first_file_data)?;
            let map_second = parse_clientside(second_file_data)?;

            for (k, v) in map_first.iter() {
                if let Some(map_value) = map_second.get(k) {
                    if !check_string_arguments(v, map_value) {
                        return Err(ArgumentMismatch(k.to_owned()));
                    }
                }
            }
        }
    }

    Ok(())
}

fn find_incomplete<T>(
    file: Option<T>,
    second_file: Option<T>,
    save_file: T,
    locale_type: LocaleType,
) -> Result<(), ParseError>
where
    T: AsRef<Path>,
{
    let data = &*read_file(file.unwrap())?;
    let occurrences: String = match locale_type {
        LocaleType::LocaleString => {
            let parsed_data = parse_data(data)?;

            find_incomplete_sections(parsed_data)
                .iter()
                .map(|s| format!("{}\n", *s))
                .collect()
        }

        LocaleType::LocaleGameInterface => {
            let second_file_data = &*read_file(second_file.unwrap())?;

            find_missing_labels(data, second_file_data)?
                .iter()
                .map(|s| format!("{}\n", *s))
                .collect()
        }
    };

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

    for s in &merged {
        if !s.check_translations_arguments() {
            return Err(ParseError::ArgumentMismatch(s.label.to_string()));
        }
    }

    let generated: String = merged
        .iter()
        .map(|f| format!("{}\n\n", f.generate()))
        .collect();

    let _ = std::fs::write(save_file, &generated)?;

    Ok(())
}
