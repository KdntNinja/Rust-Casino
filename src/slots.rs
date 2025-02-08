use crate::{
    config::Config,
    utils::{check_continue, clear_screen, get_rng, sleep_millis, sleep_secs},
};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use rand::Rng;

// Function to implement the slots game
pub fn slots(credits: &mut i32, config: &Config) {
    let mut rng = get_rng();

    // Define colored symbols
    let symbols = [
        "\x1b[31m🍒\x1b[0m", // Red Cherry
        "\x1b[33m🍊\x1b[0m", // Orange
        "\x1b[33m🍋\x1b[0m", // Yellow Lemon
        "\x1b[35m🎰\x1b[0m", // Purple Slot Machine
        "\x1b[36m💎\x1b[0m", // Cyan Diamond
        "\x1b[34m7️⃣\x1b[0m", // Blue Lucky 7
    ];

    let mut auto_roll = false;

    loop {
        clear_screen();
        println!("{}: {}", "Credits".bold(), credits);
        println!("{}", "─────────────────────────".dimmed());

        // Check if auto-roll is enabled
        if !auto_roll {
            if let Some(auto) = check_continue() {
                auto_roll = auto;
            } else {
                return; // Return to main menu if the player quits
            }
        } else {
            sleep_secs(1); // Wait for 1 second in auto-roll mode
        }

        // Deduct base points for the spin
        *credits -= config.base_points;
        let mut result = vec![""; 3];

        // Spin the slots
        for i in 0..3 {
            for _ in 0..10 {
                result[i] = symbols[rng.random_range(0..symbols.len())];
                clear_screen();
                println!("{}: {}", "Credits".bold(), credits);
                println!("{}", "─────────────────────────".dimmed());
                println!("      [{}] [{}] [{}]", result[0], result[1], result[2]);
                println!("{}", "─────────────────────────".dimmed());
                sleep_millis(100);
            }
        }

        clear_screen();
        println!("{}: {}", "Credits".bold(), credits);
        println!("{}", "─────────────────────────".dimmed());
        println!("      [{}] [{}] [{}]", result[0], result[1], result[2]);
        println!("{}", "─────────────────────────".dimmed());

        let mut show_menu = false;

        // Check for winning conditions
        if rng.random_range(0.0..1.0) < config.jackpot_probability
            && result.iter().all(|&x| x == result[0])
        {
            println!("{}", "🎉 JACKPOT! 🎉".bright_yellow().bold());
            *credits += config.jackpot_points;
            show_menu = true;
        } else if rng.random_range(0.0..1.0) < config.pair_probability
            && result.windows(2).any(|w| w[0] == w[1])
        {
            println!("{}", "🎈 You got a pair! 🎈".bright_green());
            *credits += config.pair_points;
            show_menu = true;
        } else {
            println!("{}", "Try again!".red().bold());
        }

        // Check if the player is out of credits
        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            show_menu = true;
        }

        // Show the menu if there was a win or the player is out of credits
        if show_menu {
            let options = &["Play Again", "Quit to Menu"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .items(options)
                .default(0)
                .interact()
                .expect("Failed to read selection");

            if selection == 1 {
                return; // Return to main menu if the player quits
            } else {
                auto_roll = true; // Enable auto-roll if the player chooses to play again
            }
        } else {
            if !auto_roll {
                auto_roll = true; // Enable auto-roll if the player didn't win and auto-roll is not already enabled
            }
        }
    }
}
