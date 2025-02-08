use crate::clear;
use crate::config::Config;
use colored::Colorize;

#[derive(Debug)]
struct Segment {
    number: u8,
    color: &'static str,
}

pub fn roulette(credits: &mut i32, _config: &Config) {
    loop {
        clear();
        *credits -= 1;

        // Step 1: Define the roulette wheel segments
        let wheel = init_wheel();

        // Step 2: Display the roulette wheel with colors
        for segment in &wheel {
            let colored_number = match segment.color {
                "red" => segment.number.to_string().red(),
                "black" => segment.number.to_string().black(),
                "green" => segment.number.to_string().green(),
                _ => segment.number.to_string().normal(),
            };
            print!("{} ", colored_number);
        }
        println!(); // Newline after wheel display

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

fn init_wheel() -> [Segment; 37] {
    [
        Segment {
            number: 0,
            color: "green",
        },
        Segment {
            number: 1,
            color: "red",
        },
        Segment {
            number: 2,
            color: "black",
        },
        Segment {
            number: 3,
            color: "red",
        },
        Segment {
            number: 4,
            color: "black",
        },
        Segment {
            number: 5,
            color: "red",
        },
        Segment {
            number: 6,
            color: "black",
        },
        Segment {
            number: 7,
            color: "red",
        },
        Segment {
            number: 8,
            color: "black",
        },
        Segment {
            number: 9,
            color: "red",
        },
        Segment {
            number: 10,
            color: "black",
        },
        Segment {
            number: 11,
            color: "black",
        },
        Segment {
            number: 12,
            color: "red",
        },
        Segment {
            number: 13,
            color: "black",
        },
        Segment {
            number: 14,
            color: "red",
        },
        Segment {
            number: 15,
            color: "black",
        },
        Segment {
            number: 16,
            color: "red",
        },
        Segment {
            number: 17,
            color: "black",
        },
        Segment {
            number: 18,
            color: "red",
        },
        Segment {
            number: 19,
            color: "red",
        },
        Segment {
            number: 20,
            color: "black",
        },
        Segment {
            number: 21,
            color: "red",
        },
        Segment {
            number: 22,
            color: "black",
        },
        Segment {
            number: 23,
            color: "red",
        },
        Segment {
            number: 24,
            color: "black",
        },
        Segment {
            number: 25,
            color: "red",
        },
        Segment {
            number: 26,
            color: "black",
        },
        Segment {
            number: 27,
            color: "red",
        },
        Segment {
            number: 28,
            color: "black",
        },
        Segment {
            number: 29,
            color: "black",
        },
        Segment {
            number: 30,
            color: "red",
        },
        Segment {
            number: 31,
            color: "black",
        },
        Segment {
            number: 32,
            color: "red",
        },
        Segment {
            number: 33,
            color: "black",
        },
        Segment {
            number: 34,
            color: "red",
        },
        Segment {
            number: 35,
            color: "black",
        },
        Segment {
            number: 36,
            color: "red",
        },
    ]
}
