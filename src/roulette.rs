use crate::{
    config::Config,
    utils::{clear_screen, get_rng, sleep_millis},
};
use colored::{ColoredString, Colorize};
use dialoguer::{Input, Select};
use rand::Rng;

// Structure to represent a segment on the roulette wheel
#[derive(Debug)]
struct Segment {
    number: u8,
    color: &'static str,
}

// Function to implement the roulette game
pub fn roulette(credits: &mut i32, _config: &Config) {
    loop {
        clear_screen();
        println!("{}: {}", "Credits".bold().yellow(), credits);
        println!(
            "{}",
            "─────────────────────────────────────────────────".dimmed()
        );

        // Get the bet amount from the player
        let bet: i32 = Input::new()
            .with_prompt("Enter your bet amount")
            .default(10)
            .interact_text()
            .expect("Failed to read bet");

        // Check if the player has enough credits
        if bet > *credits {
            println!("{}", "You don't have enough credits!".red().bold());
            continue;
        }

        // Select prompt with colored options
        let player_color_choice = Select::new()
            .with_prompt("Choose a colour to bet on")
            .items(&["Red".red(), "Black".black()])
            .default(0) // Default to Red
            .interact()
            .expect("Failed to read selection");

        // Match the player's selection
        let bet_color = match player_color_choice {
            0 => "red",
            1 => "black",
            _ => unreachable!(),
        };

        // Deduct the bet amount from the player's credits
        *credits -= bet;

        // Initialize the roulette wheel
        let wheel = init_wheel();
        let mut rng = get_rng();
        let result_index = rng.random_range(0..wheel.len());
        let winning_segment = &wheel[result_index];

        // Animate the wheel spin with a progress bar effect
        spin_animation_with_brackets(&wheel, result_index);

        // Display the winning number with color
        println!(
            "\n{} {}",
            "The winning number is:".bold(),
            display_number(winning_segment)
        );

        // Check if the player won
        if winning_segment.color == bet_color {
            println!("{}", "You've won your bet!".green().bold());
            *credits += bet * 2;
        } else {
            println!("{}", "You've lost your bet.".red().bold());
        }

        // Check if the player is out of credits
        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        // Ask the player if they want to play again
        let options = &["Play Again", "Quit to Menu"];
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        match selection {
            0 => continue, // Continue to the next round if they want to play again
            1 => return,   // Return to main menu if the player quits
            _ => unreachable!(),
        }
    }
}

// Function to initialize the roulette wheel segments
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
            color: "red",
        },
        Segment {
            number: 12,
            color: "black",
        },
        Segment {
            number: 13,
            color: "red",
        },
        Segment {
            number: 14,
            color: "black",
        },
        Segment {
            number: 15,
            color: "red",
        },
        Segment {
            number: 16,
            color: "black",
        },
        Segment {
            number: 17,
            color: "red",
        },
        Segment {
            number: 18,
            color: "black",
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
            color: "red",
        },
        Segment {
            number: 30,
            color: "black",
        },
        Segment {
            number: 31,
            color: "red",
        },
        Segment {
            number: 32,
            color: "black",
        },
        Segment {
            number: 33,
            color: "red",
        },
        Segment {
            number: 34,
            color: "black",
        },
        Segment {
            number: 35,
            color: "red",
        },
        Segment {
            number: 36,
            color: "black",
        },
    ]
}

// Function to display a number with its color
fn display_number(segment: &Segment) -> ColoredString {
    match segment.color {
        "red" => segment.number.to_string().red(),
        "black" => segment.number.to_string().black(),
        "green" => segment.number.to_string().green(),
        _ => segment.number.to_string().normal(),
    }
}

fn spin_animation_with_brackets(wheel: &[Segment], result_index: usize) {
    let wheel_len = wheel.len();
    let mut current_position = 0;

    while current_position != result_index {
        print!("\r");
        display_wheel_with_highlight(wheel, current_position);
        sleep_millis(100);
        current_position = (current_position + 1) % wheel_len;
    }

    print!("\r");
    display_wheel_with_highlight(wheel, result_index);
    println!();
}

fn display_wheel_with_highlight(wheel: &[Segment], highlight_index: usize) {
    for (i, segment) in wheel.iter().enumerate() {
        if i == highlight_index {
            print!("[{}] ", display_number(segment).bold().yellow()); // Highlight the segment with yellow
        } else {
            print!("{} ", display_number(segment)); // Regular display
        }
    }
    print!("\r");
}
