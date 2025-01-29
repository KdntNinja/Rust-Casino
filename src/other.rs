use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{stdout, Write};

const SLOT: [&str; 7] = ["��", "🍋", "🍊", "🍉", "🍇", "🍓", "⭐"];

fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

fn print_header() {
    println!("=====================");
    println!("  🎰 Slot Machine 🎰  ");
    println!("=====================");
}

fn print_footer() {
    println!("\n=====================");
    println!("Press Ctrl+C to exit.");
}

fn generate_3() -> [usize; 3] {
    let mut result = [0; 3];
    let mut duration = Duration::from_millis(50);
    for _ in 0..10 {
        clear_screen();
        print_header();
        for i in 0..3 {
            result[i] = rand::thread_rng().gen_range(0..7);
            print!("{} ", SLOT[result[i]]);
        }
        stdout().flush().unwrap();
        sleep(duration);
        duration += Duration::from_millis(50);
    }
    result
}

fn calculate_score(result: [usize; 3]) -> u32 {
    if result[0] == result[1] && result[1] == result[2] {
        100
    } else if result[0] == result[1] || result[1] == result[2] || result[0] == result[2] {
        50
    } else {
        0
    }
}

fn main() {
    let mut total_score = 0;
    loop {
        clear_screen();
        print_header();
        let result = generate_3();
        println!("\n{} {} {}", SLOT[result[0]], SLOT[result[1]], SLOT[result[2]]);
        let score = calculate_score(result);
        total_score += score;
        println!("\nYour score: {}", score);
        println!("Total score: {}", total_score);
        print_footer();
        sleep(Duration::from_secs(2));
    }
}
