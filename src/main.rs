use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rand::Rng;

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn menu() {
    let options = &[
        "Blackjack {WORK IN PROGREESS}".green(),
        "Poker {WORK IN PROGREESS}".blue(),
        "Roulette {WORK IN PROGREESS}".yellow(),
        "Slots".magenta(),
        "Exit".red().bold(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game")
        .items(options)
        .default(0)
        .interact()
        .expect("Failed to read selection");

    clear();

    match selection {
        0 => blackjack(),
        1 => poker(),
        2 => roulette(),
        3 => slots(),
        4 => println!("{}", "Exiting...".green().bold()),
        _ => println!("{}", "Invalid choice, please try again.".red()),
    }
}

fn blackjack() {
    println!("{}", "Starting Blackjack...".blue().bold());
    todo!();
}

fn poker() {
    println!("{}", "Starting Poker...".blue().bold());
    todo!();
}

fn roulette() {
    println!("{}", "Starting Roulette...".blue().bold());
    todo!();
}

fn slots() {
    let mut rng = rand::thread_rng();
    let symbols = ["🍒", "🍊", "🍋", "🎰", "💎", "7️⃣"];

    loop {
        clear();
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Press Enter to pull the lever (or 'q' to quit)")
            .default(" ".into())
            .interact_text()
            .expect("Failed to read input");

        if input.trim().to_lowercase() == "q" {
            println!("{}", "Thanks for playing!".green());
            break;
        }

        let result: Vec<&str> = (0..3)
            .map(|_| symbols[rng.gen_range(0..symbols.len())])
            .collect();

        println!("\n[{}][{}][{}]", result[0], result[1], result[2]);

        if result.iter().all(|&x| x == result[0]) {
            println!("{}", "🎉 JACKPOT! 🎉".bright_yellow().bold());
        } else if result.windows(2).any(|w| w[0] == w[1]) {
            println!("{}", "🎈 You got a pair! 🎈".bright_green());
        } else {
            println!("{}", "Try again!".red());
        }
    }
}

fn main() {
    menu();
}
