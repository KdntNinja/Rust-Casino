use crate::{
    config::Config,
    utils::{calculate_hand_value, clear_screen, create_deck, get_rng, sleep_millis},
};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use rand::seq::SliceRandom;

// Function to implement the blackjack game
pub fn blackjack(credits: &mut i32, config: &Config) {
    loop {
        // Create and shuffle the deck
        let mut deck = create_deck();
        let mut rng = get_rng();
        deck.shuffle(&mut rng);

        // Deal initial hands to player and dealer
        let mut player_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];
        let mut dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];

        loop {
            clear_screen();
            let player_value = calculate_hand_value(&player_hand);
            println!("Credits: {}", credits);
            println!("{}", "Your hand:".bold());
            println!("{}", format_hand(&player_hand).bright_blue());
            println!("Total value: {}", player_value);
            println!();
            println!("{}", "Dealer's hand:".bold());
            println!("[{}, ?]", dealer_hand[0].bright_red());
            println!();

            // Get player's action: Hit or Stand
            let options = &["Hit".green(), "Stand".red()];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an action")
                .items(options)
                .default(0)
                .interact()
                .expect("Failed to read selection");

            match selection {
                0 => {
                    // Player chooses to hit
                    let new_card = deck.pop().unwrap();
                    player_hand.push(new_card.clone());
                    animate_hand(&player_hand, credits, &dealer_hand, &new_card);

                    // Check if player busted
                    if calculate_hand_value(&player_hand) > 21 {
                        clear_screen();
                        println!("{}", "You busted!".red().bold());
                        *credits -= config.blackjack_loss_points;
                        break;
                    }
                }
                1 => {
                    // Player stands, dealer's turn
                    while calculate_hand_value(&dealer_hand) < 17 {
                        let new_card = deck.pop().unwrap();
                        dealer_hand.push(new_card.clone());
                        animate_hand(&dealer_hand, credits, &dealer_hand, &new_card);
                    }

                    let player_value = calculate_hand_value(&player_hand);
                    let dealer_value = calculate_hand_value(&dealer_hand);

                    clear_screen();
                    println!("Credits: {}", credits);
                    println!("{}", "Your hand:".bold());
                    println!("{}", format_hand(&player_hand).bright_blue());
                    println!("Total value: {}", player_value);
                    println!();
                    println!("{}", "Dealer's hand:".bold());
                    println!("{}", format_hand(&dealer_hand).bright_red());
                    println!("Total value: {}", dealer_value);
                    println!();

                    // Determine the winner
                    if dealer_value > 21 || player_value > dealer_value {
                        println!("{}", "You win!".green().bold());
                        *credits += config.blackjack_win_points;
                    } else if player_value < dealer_value {
                        println!("{}", "You lose!".red().bold());
                        *credits -= config.blackjack_loss_points;
                    } else {
                        println!("{}", "It's a tie!".yellow().bold());
                    }
                    break; // End the inner loop after dealer's turn
                }
                _ => println!("{}", "Invalid choice, please try again.".red()),
            }
        }

        // Ask the player if they want to play again
        let options = &["Play Again", "Quit to Menu"];
        let selection = Select::with_theme(&ColorfulTheme::default())
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

// Function to format a hand of cards into a string
fn format_hand(hand: &[String]) -> String {
    hand.join(", ")
}

// Function to animate dealing a card
fn animate_hand(hand: &[String], credits: &i32, dealer_hand: &[String], new_card: &String) {
    clear_screen();
    let player_value = calculate_hand_value(hand);
    println!("Credits: {}", credits);
    println!("{}", "Your hand:".bold());
    println!("{}", format_hand(hand).bright_blue());
    println!("Total value: {}", player_value);
    println!();
    println!("{}", "Dealer's hand:".bold());
    println!("[{}, ?]", dealer_hand[0].bright_red());
    println!();
    println!("New card: {}", new_card.bright_green());
    sleep_millis(200);
}
