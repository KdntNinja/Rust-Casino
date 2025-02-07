use crate::config::Config;
use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use dialoguer::{theme::ColorfulTheme, Select};
use rand::rng;
use rand::seq::SliceRandom;

pub fn blackjack(credits: &mut i32, config: &Config) {
    println!("{}", "Starting Blackjack...".blue().bold());

    let mut deck = create_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);

    let mut player_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let mut dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];

    loop {
        clear();
        let player_value = calculate_hand_value(&player_hand);
        println!("Credits: {}", credits);
        println!("{}", "Your hand:".bold());
        println!("{}", format_hand(&player_hand).bright_blue());
        println!("Total value: {}", player_value);
        println!();
        println!("{}", "Dealer's hand:".bold());
        println!("[{}, ?]", dealer_hand[0].bright_red());
        println!();

        let options = &["Hit".green(), "Stand".red()];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .items(options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        match selection {
            0 => {
                player_hand.push(deck.pop().unwrap());
                if calculate_hand_value(&player_hand) > 21 {
                    clear();
                    println!("Credits: {}", credits);
                    println!("{}", "Your hand:".bold());
                    println!("{}", format_hand(&player_hand).bright_blue());
                    println!("Total value: {}", calculate_hand_value(&player_hand));
                    println!();
                    println!("{}", "Dealer's hand:".bold());
                    println!("[{}, ?]", dealer_hand[0].bright_red());
                    println!();
                    println!("{}", "You busted!".red().bold());
                    *credits -= config.blackjack_loss_points;
                    break;
                }
            }
            1 => {
                while calculate_hand_value(&dealer_hand) < 17 {
                    dealer_hand.push(deck.pop().unwrap());
                }
                let player_value = calculate_hand_value(&player_hand);
                let dealer_value = calculate_hand_value(&dealer_hand);

                clear();
                println!("Credits: {}", credits);
                println!("{}", "Your hand:".bold());
                println!("{}", format_hand(&player_hand).bright_blue());
                println!("Total value: {}", player_value);
                println!();
                println!("{}", "Dealer's hand:".bold());
                println!("{}", format_hand(&dealer_hand).bright_red());
                println!("Total value: {}", dealer_value);
                println!();

                if dealer_value > 21 || player_value > dealer_value {
                    println!("{}", "You win!".green().bold());
                    *credits += config.blackjack_win_points;
                } else if player_value < dealer_value {
                    println!("{}", "You lose!".red().bold());
                    *credits -= config.blackjack_loss_points;
                } else {
                    println!("{}", "It's a tie!".yellow().bold());
                }
                break;
            }
            _ => println!("{}", "Invalid choice, please try again.".red()),
        }
    }

    println!("Credits: {}", credits);
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
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}
