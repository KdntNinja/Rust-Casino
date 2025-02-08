use crate::config::Config;
use colored::Colorize;

pub fn roulette(credits: &mut i32, config: &Config) {
    loop {
        println!("{}", "Starting Roulette...".blue().bold());
        // Placeholder for actual roulette game logic
        // ...

        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        let options = &["Play Again", "Quit to Menu"];
        let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
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
