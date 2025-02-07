use crate::{clear, config::Config};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input};
use rand::Rng;
use std::{thread, time::Duration};

pub fn slots(credits: &mut i32, config: &Config) {
    let mut rng = rand::rng();
    let symbols = ["🍒", "🍊", "🍋", "🎰", "💎", "7️⃣"];
    let mut auto_roll = false;

    loop {
        clear();
        println!("Credits: {}", credits);

        if !auto_roll {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Press Enter to pull the lever (or 'q' to quit, 'a' for auto-roll)")
                .default(" ".into())
                .interact_text()
                .expect("Failed to read input");

            if input.trim().to_lowercase() == "q" {
                println!("{}", "Thanks for playing!".green());
                break;
            } else if input.trim().to_lowercase() == "a" {
                auto_roll = true;
            }
        } else {
            thread::sleep(Duration::from_secs(1));
        }

        *credits -= config.base_points;

        let mut result = vec![""; 3];
        for i in 0..3 {
            for _ in 0..10 {
                result[i] = symbols[rng.random_range(0..symbols.len())];
                clear();
                println!("Credits: {}", credits);
                println!("\n[{}][{}][{}]", result[0], result[1], result[2]);
                thread::sleep(Duration::from_millis(100));
            }
        }

        if result.iter().all(|&x| x == result[0]) {
            println!("{}", "🎉 JACKPOT! 🎉".bright_yellow().bold());
            *credits += config.jackpot_points;
            auto_roll = false;
        } else if result.windows(2).any(|w| w[0] == w[1]) {
            println!("{}", "🎈 You got a pair! 🎈".bright_green());
            *credits += config.pair_points;
            auto_roll = false;
        } else {
            println!("{}", "Try again!".red());
        }

        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        if !auto_roll {
            println!("Press Enter to play again...");
            let check_continue: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("")
                .default(" ".into())
                .interact_text()
                .expect("Failed to read input");

            if check_continue.trim().to_lowercase() == "q" {
                println!("{}", "Thanks for playing!".green());
                break;
            } else if check_continue.trim().to_lowercase() == "a" {
                auto_roll = true;
            }
        }
    }
}
