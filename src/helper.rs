use colored::Colorize;

pub fn check_continue() -> Option<bool> {
    use dialoguer::{theme::ColorfulTheme, Input};

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Press Enter to continue, 'q' to quit, 'a' for auto-roll")
        .default(" ".into())
        .interact_text()
        .expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "q" => {
            println!("{}", "Thanks for playing!".green().bold());
            None // Indicates the player wants to quit
        }
        "a" => Some(true), // Enable auto-roll
        _ => Some(false),  // Continue playing manually
    }
}

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
