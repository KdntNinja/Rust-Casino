use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::{thread, time::Duration};

// Initialize the random number generator as a static variable
lazy_static::lazy_static! {
    static ref RNG: std::sync::Mutex<StdRng> = std::sync::Mutex::new(StdRng::from_seed([0u8; 32]));
}

// Function to get a mutable reference to the random number generator
pub fn get_rng() -> std::sync::MutexGuard<'static, StdRng> {
    RNG.lock().unwrap()
}

// Function to clear the terminal screen
pub fn clear_screen() {
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

// Function to pause the execution for a specified number of milliseconds
pub fn sleep_millis(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

// Function to pause the execution for a specified number of seconds
pub fn sleep_secs(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

// Function to prompt the user to continue or quit
pub fn check_continue() -> Option<bool> {
    use dialoguer::{theme::ColorfulTheme, Input};

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Press Enter to continue, 'q' to quit")
        .default("".into())
        .interact_text()
        .expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "q" => {
            println!("{}", "Thanks for playing!".green().bold());
            None // Indicates the player wants to quit
        }
        _ => Some(true), // Enable auto-roll
    }
}

// Function to create a standard deck of cards
pub fn create_deck() -> Vec<String> {
    let suits = ["♠", "♥", "♦", "♣"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let mut deck = Vec::new();
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(format!("{}{}", rank, suit));
        }
    }
    deck
}

// Function to calculate the value of a hand in Blackjack
pub fn calculate_hand_value(hand: &[String]) -> i32 {
    let mut value = 0;
    let mut aces = 0;

    for card in hand {
        let rank = card
            .chars()
            .take_while(|c| c.is_digit(10) || *c == 'J' || *c == 'Q' || *c == 'K' || *c == 'A')
            .collect::<String>();
        value += match rank.as_str() {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" | "Q" | "K" => 10,
            "A" => {
                aces += 1;
                11
            }
            _ => 0,
        };
    }
    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }
    value
}
