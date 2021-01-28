#![allow(dead_code)]
#![allow(unused_variables)]
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

mod parse;
mod section;

fn main() -> std::io::Result<()> {
    println!("Choose action:");

    let select_items = vec![
        "Merge translations",
        "Convert old file",
        "Find missing translations",
    ];

    let option = Select::with_theme(&ColorfulTheme::default())
        .items(&select_items)
        .default(0)
        .interact();

    match option {
        Ok(index) => {
            match index {
                0 => {
                    let cur_file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the filename containing current translations")
                        .default("locale_string.txt".into())
                        .interact_text()?;

                    let newer_file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the filename containing newer translations")
                        .default("locale_string2.txt".into())
                        .interact_text()?;

                    let save_file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the filename to which new translations will be saved")
                        .default("locale_string_new.txt".into())
                        .interact_text()?;

                    // merge
                }
                1 => {
                    let old_file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the filename containing old translations")
                        .default("locale_string.txt".into())
                        .interact_text()?;

                    let new_file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt(
                            "Enter the filename to which converted translations will be saved",
                        )
                        .default("locale_string_new.txt".into())
                        .interact_text()?;

                    // convert
                }
                2 => {
                    let file: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the filename containing translations")
                        .default("locale_string.txt".into())
                        .interact_text()?;
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}
