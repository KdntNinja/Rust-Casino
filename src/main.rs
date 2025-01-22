use std::io;

fn main() {
    loop {
        let mut number: String = String::new();
        println!("Enter a number: ");
        io::stdin().read_line(&mut number).expect("Failed to read line");

        let mut power: String = String::new();
        println!("Enter the power: ");
        io::stdin().read_line(&mut power).expect("Failed to read line");

        let number: i64 = number.trim().parse().expect("Please type a number!");
        let power: u32 = power.trim().parse().expect("Please type a number!");
        let answer: i64 = number.pow(power);
        print!("{} to the power of {} is: {}", number, power, answer);

        let mut choice: String = String::new();
        println!("\nDo you want to continue? (y/n)");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        if choice.trim() == "y" {
            continue;
        } else if choice.trim() == "n" {
            break;
        } else {
            println!("Invalid choice. Exiting the program.");
            break;
        }
    }
}
