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
