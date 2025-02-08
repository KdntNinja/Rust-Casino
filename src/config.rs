use serde::{Deserialize, Serialize};
use std::fs;

// Structure to hold the configuration parameters
#[derive(Deserialize, Serialize)]
pub struct Config {
    // Points awarded for a jackpot in slots
    pub jackpot_points: i32,
    // Points awarded for a pair in slots
    pub pair_points: i32,
    // Base points deducted for each spin in slots
    pub base_points: i32,
    // Initial credits given to the player
    pub base_credits: i32,
    // Points awarded for winning in blackjack
    pub blackjack_win_points: i32,
    // Points deducted for losing in blackjack
    pub blackjack_loss_points: i32,
    // Probability of hitting a jackpot in slots
    pub jackpot_probability: f64,
    // Probability of getting a pair in slots
    pub pair_probability: f64,
}

// Implement the Default trait for the Config struct
impl Default for Config {
    fn default() -> Self {
        Config {
            jackpot_points: 100,
            pair_points: 10,
            base_points: 1,
            base_credits: 50,
            blackjack_win_points: 10,
            blackjack_loss_points: 10,
            jackpot_probability: 0.05,
            pair_probability: 0.15,
        }
    }
}

// Function to load the configuration from a JSON file
pub fn load_config() -> Config {
    let config_data = fs::read_to_string("config.json").unwrap_or_else(|_| {
        println!("Config file not found, using default values.");
        return serde_json::to_string(&Config::default()).unwrap();
    });
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}
