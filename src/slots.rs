use crate::{clear, config::Config, helper::check_continue};
use colored::Colorize;
use rand::Rng;
use std::{thread, time::Duration};

pub fn slots(credits: &mut i32, config: &Config) {
    let mut rng = rand::rng();

    // Define colored symbols using ANSI escape codes
    let symbols = [
        "\x1b[31m🍒\x1b[0m", // Red Cherry
        "\x1b[33m🍊\x1b[0m", // Orange
        "\x1b[33m🍋\x1b[0m", // Yellow Lemon
        "\x1b[35m🎰\x1b[0m", // Purple Slot Machine
        "\x1b[36m💎\x1b[0m", // Cyan Diamond
        "\x1b[34m7️\x1b[0m", // Blue Lucky 7
    ];

    let mut auto_roll = false;

    loop {
        clear();
        println!("{}: {}", "Credits".bold(), credits);
        println!("{}", "─────────────────────────".dimmed());

        if !auto_roll {
            if let Some(auto) = check_continue() {
                auto_roll = auto;
            } else {
                break;
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
                println!("{}: {}", "Credits".bold(), credits);
                println!("{}", "─────────────────────────".dimmed());
                println!("      [{}] [{}] [{}]", result[0], result[1], result[2]);
                println!("{}", "─────────────────────────".dimmed());
                thread::sleep(Duration::from_millis(100));
            }
        }

        clear();
        println!("{}: {}", "Credits".bold(), credits);
        println!("{}", "─────────────────────────".dimmed());
        println!("      [{}] [{}] [{}]", result[0], result[1], result[2]);
        println!("{}", "─────────────────────────".dimmed());

        if result.iter().all(|&x| x == result[0]) {
            println!("{}", "🎉 JACKPOT! 🎉".bright_yellow().bold());
            *credits += config.jackpot_points;
            auto_roll = false;
        } else if result.windows(2).any(|w| w[0] == w[1]) {
            println!("{}", "🎈 You got a pair! 🎈".bright_green());
            *credits += config.pair_points;
            auto_roll = false;
        } else {
            println!("{}", "Try again!".red().bold());
        }

        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        if !auto_roll {
            if let Some(auto) = check_continue() {
                auto_roll = auto;
            } else {
                return; // Return to main menu
            }
        }
    }
}
