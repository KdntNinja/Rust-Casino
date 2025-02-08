use crate::clear;
use crate::config::Config;
use colored::{ColoredString, Colorize};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rand::Rng;
use std::{thread, time};

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

        // Step 2: Display the roulette wheel with numbers (0 to 36)
        display_wheel(&wheel);

        // Step 3: Allow the player to place a bet
        let player_bet = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter your bet amount: ")
            .default(10)
            .interact()
            .expect("Failed to enter bet amount.");

        // Step 4: Allow the player to choose a color to bet on
        let color_options = &["Red", "Black", "Green"];
        let player_color_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a color to bet on")
            .items(color_options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        let bet_color = match player_color_choice {
            0 => "red",
            1 => "black",
            2 => "green",
            _ => unreachable!(),
        };

        // Step 5: Spin the roulette wheel and animate the wheel spin with brackets
        let mut rng = rand::rng();
        let wheel_len = wheel.len();

        // Animate the wheel spin with selection brackets
        spin_animation_with_brackets(&wheel);

        // Randomly select the result
        let result_index = rng.random_range(0..wheel_len);
        let winning_segment = &wheel[result_index];

        // Show the result after spinning
        println!(
            "\nThe winning number is: {}",
            display_number(winning_segment)
        );

        // Step 6: Check if the player's bet on color is correct
        if winning_segment.color == bet_color {
            println!("{}", "You've won your bet!".green());
            *credits += player_bet * 2; // Double the bet for correct color
        } else {
            println!("{}", "You've lost your bet.".red());
        }

        // Step 7: Check if the player wants to play again or quit
        if *credits <= 0 {
            println!("{}", "Out of credits! Game over.".red().bold());
            break;
        }

        let options = &["Play Again", "Quit to Menu"];
        let selection = Select::with_theme(&ColorfulTheme::default())
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

fn display_number(segment: &Segment) -> ColoredString {
    match segment.color {
        "red" => segment.number.to_string().red(),
        "black" => segment.number.to_string().black(),
        "green" => segment.number.to_string().green(),
        _ => segment.number.to_string().normal(),
    }
}

fn display_wheel(wheel: &[Segment]) {
    // Display numbers from 0 to 36 in a row
    for segment in wheel {
        print!("{} ", display_number(segment));
    }
    println!(); // Newline after wheel display
}

fn spin_animation_with_brackets(wheel: &[Segment]) {
    let wheel_len = wheel.len();

    // Run the animation loop until the selected number is reached
    let mut rng = rand::rng();
    let result_index = rng.random_range(0..wheel_len);

    let mut current_position = 0;

    // Loop the selection animation until the winning number is reached
    while current_position != result_index {
        print!("\r");

        // Display the numbers and add brackets around the current selection
        for (i, segment) in wheel.iter().enumerate() {
            if i == current_position {
                print!("[{}] ", display_number(segment));
            } else {
                print!("{} ", display_number(segment));
            }
        }

        // Wait for a quick moment to create animation effect
        thread::sleep(time::Duration::from_millis(100)); // Fast movement
        current_position = (current_position + 1) % wheel_len; // Loop through the positions
    }

    // Final display of the winning number
    print!("\r");
    for (i, segment) in wheel.iter().enumerate() {
        if i == result_index {
            print!("[{}] ", display_number(segment)); // Bracket around the winning number
        } else {
            print!("{} ", display_number(segment));
        }
    }
    println!(); // Move to the next line after the animation
}
