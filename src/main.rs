use dialoguer::{Input, Confirm};

fn main() {
    let mut count: i32 = 0;
    let mut sum: i32 = 0;
    
    loop {
        let number: i32 = match get_number() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
        count += 1;
        sum += number;

        if !ask_to_continue() {
            break;
        }
    }

    println!("Count: {}", count);
    println!("Sum: {}", sum);
}

fn get_number() -> Result<i32, String> {
    let input: String = Input::new()
        .with_prompt("Enter a number")
        .interact_text()
        .map_err(|_| "Failed to read input".to_string())?;
    input.trim().parse().map_err(|_| "Please enter a valid number".to_string())
}

fn ask_to_continue() -> bool {
    Confirm::new()
        .with_prompt("Do you want another go?")
        .default(false)
        .interact()
        .unwrap_or(false)
}
