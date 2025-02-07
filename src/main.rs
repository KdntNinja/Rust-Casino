mod slots;

use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
struct Config {
    jackpot_points: i32,
    pair_points: i32,
    base_points: i32,
    base_credits: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            jackpot_points: 100,
            pair_points: 10,
            base_points: 1,
            base_credits: 50,
        }
    }
}

fn load_config() -> Config {
    let config_data = fs::read_to_string("config.json").unwrap_or_else(|_| {
        println!("Config file not found, using default values.");
        return serde_json::to_string(&Config::default()).unwrap();
    });
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}

fn clear() {
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

fn menu(credits: &mut i32, config: &Config) {
    clear();
    let options = &[
        "Slots".magenta(),
        "Blackjack {WORK IN PROGRESS}".green(),
        "Poker {WORK IN PROGRESS}".blue(),
        "Roulette {WORK IN PROGRESS}".yellow(),
        "Exit".red().bold(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game")
        .items(options)
        .default(0)
        .interact()
        .expect("Failed to read selection");

    match selection {
        0 => slots::slots(credits, config),
        1 => blackjack(credits),
        2 => poker(credits),
        3 => roulette(credits),
        4 => println!("{}", "Exiting...".green().bold()),
        _ => println!("{}", "Invalid choice, please try again.".red()),
    }
}

fn blackjack(_credits: &mut i32) {
    println!("{}", "Starting Blackjack...".blue().bold());
    todo!();
}

fn poker(_credits: &mut i32) {
    println!("{}", "Starting Poker...".blue().bold());
    todo!();
}

fn roulette(_credits: &mut i32) {
    println!("{}", "Starting Roulette...".blue().bold());
    todo!();
}

fn main() {
    let config: Config = load_config();
    let mut credits = config.base_credits;
    menu(&mut credits, &config);
}
