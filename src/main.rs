mod blackjack;
mod config;
mod poker;
mod roulette;
mod slots;

use colored::Colorize;
use config::{load_config, Config};
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use dialoguer::{theme::ColorfulTheme, Select};

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
        1 => blackjack::blackjack(credits, config),
        2 => poker::poker(credits),
        3 => roulette::roulette(credits),
        4 => println!("{}", "Exiting...".green().bold()),
        _ => println!("{}", "Invalid choice, please try again.".red()),
    }
}

fn main() {
    let config: Config = load_config();
    let mut credits = config.base_credits;
    menu(&mut credits, &config);
}
