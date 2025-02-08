use crate::config::Config;
use colored::Colorize;

pub fn roulette(credits: &mut i32, _config: &Config) {
    loop {
        // Step 1: Define the roulette wheel segments
        // - Create an array or vector to represent the segments of the roulette wheel.
        // - Each segment can be a number or a color (e.g., red, black, green).

        // Step 2: Display the roulette wheel
        // - Use terminal graphics or simple text to display the roulette wheel.
        // - Highlight the current position of the wheel.

        // Step 3: Allow the player to place a bet
        // - Prompt the player to enter their bet amount.
        // - Prompt the player to choose a segment to bet on (e.g., a number or color).

        // Step 4: Spin the roulette wheel
        // - Randomly select a segment as the result of the spin.
        // - Animate the spinning of the wheel for a more engaging experience.

        // Step 5: Determine the outcome
        // - Compare the result of the spin with the player's bet.
        // - Update the player's credits based on whether they won or lost.

        // Step 6: Display the result
        // - Show the result of the spin and whether the player won or lost.
        // - Update the display of the player's credits.

        // Step 7: Check if the player wants to play again or quit
        // - Prompt the player to choose whether to play again or return to the main menu.

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
            return;
        }
    }
}
