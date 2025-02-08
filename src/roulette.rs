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
        println!("{}: {}", "Credits".bold(), credits);
        println!("{}", "─────────────────────────".dimmed());

        // Get the bet amount from the player
        let bet: i32 = Input::new()
            .with_prompt("Enter your bet amount")
            .interact_text()
            .expect("Failed to read bet");

        // Check if the player has enough credits
        if bet > *credits {
            println!("{}", "You don't have enough credits!".red().bold());
            continue;
        }

        // Get the color choice from the player
        let color_options = &["Red", "Black", "Green"];
        let player_color_choice = Select::new()
            .with_prompt("Choose a color to bet on")
            .items(color_options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        // Match the player's color choice to a string
        let bet_color = match player_color_choice {
            0 => "red",
            1 => "black",
            2 => "green",
            _ => unreachable!(),
        };

        // Deduct the bet amount from the player's credits
        *credits -= bet;

        // Initialize the roulette wheel
        let wheel = init_wheel();
        let mut rng = get_rng();
        let result_index = rng.random_range(0..wheel.len());
        let winning_segment = &wheel[result_index];

        // Animate the wheel spin
        spin_animation_with_brackets(&wheel, result_index);

        // Display the winning number
        println!(
            "\nThe winning number is: {}",
            display_number(winning_segment)
        );

        // Check if the player won
        if winning_segment.color == bet_color {
            println!("{}", "You've won your bet!".green());
            *credits += bet * 2;
        } else {
            println!("{}", "You've lost your bet.".red());
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

        if selection == 1 {
            return; // Return to main menu if the player quits
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

// Function to display a number with its color
fn display_number(segment: &Segment) -> ColoredString {
    match segment.color {
        "red" => segment.number.to_string().red(),
        "black" => segment.number.to_string().black(),
        "green" => segment.number.to_string().green(),
        _ => segment.number.to_string().normal(),
    }
}

// Function to animate the wheel spin with brackets
fn spin_animation_with_brackets(wheel: &[Segment], result_index: usize) {
    let wheel_len = wheel.len();
    let mut current_position = 0;

    while current_position != result_index {
        print!("\r");
        for (i, segment) in wheel.iter().enumerate() {
            if i == current_position {
                print!("[{}] ", display_number(segment));
            } else {
                print!("{} ", display_number(segment));
            }
        }
        sleep_millis(100);
        current_position = (current_position + 1) % wheel_len;
    }

    print!("\r");
    for (i, segment) in wheel.iter().enumerate() {
        if i == result_index {
            print!("[{}] ", display_number(segment));
        } else {
            print!("{} ", display_number(segment));
        }
    }
    println!();
}
