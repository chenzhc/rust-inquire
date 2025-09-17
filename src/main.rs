#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::NaiveDate;
use colorize::AnsiColor;
use inquire::{validator::{ErrorMessage, Validation}, CustomUserError, DateSelect, MultiSelect, Select};
use log::info;

fn main() {
    rust_inquire::init();
    // println!("{}", );
    // prompt_text();
    // prompt_select();
    // prompt_multiselect();
    let player = Player {
        name: prompt_text(),
        birth_date: prompt_date(),
        classes: prompt_multiselect(),
    };

    info!("{:?}", player);
    
}

#[derive(Debug)]
struct Player {
    name: String,
    classes: Vec<String>,
    birth_date: NaiveDate,
}

fn prompt_date() -> NaiveDate {
    let prompt_message = "Select the player's birth date";
    let selcted_date = DateSelect::new(prompt_message)
        .prompt()
        .expect("Failed to select player's birth date");

    info!("You selected {}", selcted_date.format("%Y month: %m day: %d"));

    return selcted_date;
}


fn prompt_multiselect() -> Vec<String> {
    let prompt_message = "Please select your player classes?".yellow();
    let player_classes = vec![
        "Druid".to_string(),
        "Cleric".to_string(),
        "Archer".to_string(),
        "Warrior".to_string(),
        "Wizard".to_string(),
    ];

    let select = MultiSelect::new(&prompt_message, player_classes)
        .prompt()
        .expect("Failed to select multiple player class");

    // for sel in select {
    //     info!("You selected the player class {}", sel.yellow());
    // }

    return select;
}

fn prompt_select() -> String {
    let prompt_message = "Please select your player classes?".yellow();
    let player_classes = vec![
        "Druid",
        "Cleric",
        "Archer",
        "Warrior",
        "Wizard",
    ];

    let select = Select::new(&prompt_message, player_classes)
        .prompt()
        .expect("Failed to select player class");

    info!("You selected the player class {}", select.yellow());
    return select.to_string();
}

fn prompt_text() -> String {

    let name_validator = |i: &str | -> Result<Validation, CustomUserError> {
        let first_char = i.chars().next().unwrap() as u8;
        match first_char {
            65..=90 => {
                return Ok(Validation::Valid);
            },
            _ => { 
                let err_msg = "Please make sure the first character is capital Latter";
                return Ok(Validation::Invalid(err_msg.into()))
            }
        }
    };
    let prompt_message = "What is your player name?".yellow();
    let player_name = inquire::Text::new(&prompt_message)
        .with_validator(name_validator)
        .prompt()
        .expect("Failed to capture player name");
    // let player_name = inquire::prompt_text(prompt_message)
    //     .expect("Failed to capture player name");

    // info!("Your player name is: {}", player_name.yellow());
    return player_name;
}

fn prompt_boolean() {
    let message = "Are you ready to proceed?".yellow();

    let proceed = inquire::prompt_confirmation(message);
    if proceed.is_err() {
        info!("Error occurred while checking if to procced");
    }

    if proceed.unwrap() {
        info!("User selected to proceed with application ");
    } else {
        info!("{}", "User is not okay with proceeding".yellow());
    }
}