mod blackjack;
mod config;
mod helper;
mod poker;
mod roulette;
mod slots;

use colored::Colorize;
use config::{load_config, Config};
use dialoguer::{console::Style, theme::ColorfulTheme, Select};
use helper::clear;

fn menu(credits: &mut i32, config: &Config) {
    loop {
        clear();
        let options = &[
            "Slots".magenta(),
            "Blackjack".green(),
            "Poker".blue(),
            "Roulette {WORK IN PROGRESS}".yellow(),
            "Exit".red().bold(),
        ];

        let theme = ColorfulTheme {
            active_item_style: Style::new().for_stderr().bold().on_bright(),
            inactive_item_style: Style::new().for_stderr().dim(),
            active_item_prefix: Style::new()
                .for_stderr()
                .white()
                .bold()
                .apply_to(">>".to_string()),
            ..ColorfulTheme::default()
        };

        let selection = Select::with_theme(&theme)
            .with_prompt("Select a game")
            .items(options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        match selection {
            0 => slots::slots(credits, config),
            1 => blackjack::blackjack(credits, config),
            2 => poker::poker(credits, config),
            3 => roulette::roulette(credits, config),
            4 => {
                println!("{}", "Exiting...".green().bold());
                break;
            }
            _ => println!("{}", "Invalid choice, please try again.".red()),
        }
    }
}

fn main() {
    let config: Config = load_config();
    let mut credits = config.base_credits;
    menu(&mut credits, &config);
}
