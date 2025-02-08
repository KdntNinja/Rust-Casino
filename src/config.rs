use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub jackpot_points: i32,
    pub pair_points: i32,
    pub base_points: i32,
    pub base_credits: i32,
    pub blackjack_win_points: i32,
    pub blackjack_loss_points: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            jackpot_points: 100,
            pair_points: 10,
            base_points: 1,
            base_credits: 50,
            blackjack_win_points: 10,
            blackjack_loss_points: 10,
        }
    }
}

pub fn load_config() -> Config {
    let config_data = fs::read_to_string("config.json").unwrap_or_else(|_| {
        println!("Config file not found, using default values.");
        return serde_json::to_string(&Config::default()).unwrap();
    });
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}
