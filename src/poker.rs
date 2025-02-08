use crate::config::Config;
use crate::helper::{calculate_hand_value, create_deck};
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

        let mut player_hand = deal_hand(&mut deck);
        let bot_hand = deal_hand(&mut deck);

        display_hands(credits, bet, &player_hand, &bot_hand);

        // Allow player to discard and draw new cards
        let discard_indices: Vec<usize> = MultiSelect::new()
            .with_prompt("Select cards to discard")
            .items(&player_hand)
            .interact()
            .expect("Failed to read selection");

        discard_and_draw_cards(&mut player_hand, &mut deck, &discard_indices);

        clear();
        display_hands(credits, bet, &player_hand, &bot_hand);

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

fn deal_hand(deck: &mut Vec<String>) -> Vec<String> {
    vec![
        deck.pop().unwrap(),
        deck.pop().unwrap(),
        deck.pop().unwrap(),
        deck.pop().unwrap(),
        deck.pop().unwrap(),
    ]
}

fn discard_and_draw_cards(
    player_hand: &mut Vec<String>,
    deck: &mut Vec<String>,
    discard_indices: &[usize],
) {
    for &index in discard_indices.iter().rev() {
        player_hand.remove(index);
    }

    while player_hand.len() < 5 {
        player_hand.push(deck.pop().unwrap());
    }
}

fn display_hands(credits: &i32, bet: i32, player_hand: &[String], bot_hand: &[String]) {
    clear();
    println!("Credits: {}", credits);
    println!("Bet: {}", bet);
    println!("{}", "Your hand:".bold());
    println!("{}", format_hand(player_hand).bright_blue());
    println!();
    println!("{}", "Bot's hand:".bold());
    println!("{}", format_hand(bot_hand).bright_red());
    println!();
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
