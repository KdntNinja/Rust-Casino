use std::io;

const PASSWORD: &str = "password";

fn main() {
    let mut attempts: i16 = 0;
    while attempts < 3 {
        if check_password() {
            println!("Correct password!");
            break;
        } else {
            println!("Incorrect password! {} attempts left", 2 - attempts);
            println!();
            attempts += 1;
        }
    }
}

fn check_password() -> bool {
    println!("Enter the password: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    input == PASSWORD
}