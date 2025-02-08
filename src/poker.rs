use crate::config::Config;
use colored::Colorize;
use dialoguer::{Input, MultiSelect, Select};
use rand::rng;
use rand::seq::SliceRandom;

pub fn poker(credits: &mut i32, _config: &Config) {
    loop {
        let mut deck = create_deck();
        let mut rng = rng();
        deck.shuffle(&mut rng);

        let bet: i32 = Input::new()
            .with_prompt("Enter your bet")
            .interact_text()
            .expect("Failed to read bet");

        if bet > *credits {
            println!("{}", "You don't have enough credits!".red().bold());
            continue;
        }

        let mut player_hand = vec![
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
        ];
        let mut bot_hand = vec![
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
        ];

        clear();
        println!("Credits: {}", credits);
        println!("Bet: {}", bet);
        println!("{}", "Your hand:".bold());
        println!("{}", format_hand(&player_hand).bright_blue());
        println!();
        println!("{}", "Bot's hand:".bold());
        println!("{}", format_hand(&bot_hand).bright_red());
        println!();

        // Allow player to discard and draw new cards
        let discard_indices: Vec<usize> = MultiSelect::new()
            .with_prompt("Select cards to discard")
            .items(&player_hand)
            .interact()
            .expect("Failed to read selection");

        for &index in discard_indices.iter().rev() {
            player_hand.remove(index);
        }

        while player_hand.len() < 5 {
            player_hand.push(deck.pop().unwrap());
        }

        clear();
        println!("Credits: {}", credits);
        println!("Bet: {}", bet);
        println!("{}", "Your new hand:".bold());
        println!("{}", format_hand(&player_hand).bright_blue());
        println!();
        println!("{}", "Bot's hand:".bold());
        println!("{}", format_hand(&bot_hand).bright_red());
        println!();

        // Simple win/lose logic for demonstration
        if calculate_hand_value(&player_hand) > calculate_hand_value(&bot_hand) {
            println!("{}", "You win!".green().bold());
            *credits += bet; // Win the bet amount
        } else {
            println!("{}", "You lose!".red().bold());
            *credits -= bet; // Lose the bet amount
        }

        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        let options = &["Play Again", "Quit to Menu"];
        let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        if selection == 1 {
            return; // Return to main menu
        }
    }
}

fn create_deck() -> Vec<String> {
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

fn calculate_hand_value(hand: &[String]) -> i32 {
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

fn format_hand(hand: &[String]) -> String {
    hand.join(", ")
}

fn clear() {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )
    .unwrap();
}
