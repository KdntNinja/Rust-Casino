use colored::Colorize;
use fastnoise_lite::{NoiseType, FastNoiseLite};

pub struct BlockType {
    pub symbol: &'static str,
    pub color: colored::Color,
}

impl BlockType {
    pub const fn new(symbol: &'static str, color: colored::Color) -> Self {
        BlockType { symbol, color }
    }

    pub fn color(&self) -> colored::ColoredString {
        self.symbol.color(self.color)
    }
}

// Constants for block types and colors
pub const AIR: BlockType = BlockType::new(" ", colored::Color::White);
pub const WATER: BlockType = BlockType::new("🟦", colored::Color::Blue);
pub const GRASS: BlockType = BlockType::new("🟩", colored::Color::Green);
pub const DIRT: BlockType = BlockType::new("🟫", colored::Color::Yellow);
pub const STONE: BlockType = BlockType::new("⬛", colored::Color::BrightBlack);

// Function to determine the block type based on coordinates and seed
pub fn set_block(x: f64, y: usize, z: f64, seed: u64) -> colored::ColoredString {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_seed(Some(seed as i32));
    let height = (noise.get_noise_2d(x as f32 / 100.0, z as f32 / 100.0) * 50.0 + 100.0) as usize;

    if y < height - 10 {
        STONE.color()
    } else if y < height - 5 {
        DIRT.color()
    } else if y < height {
        GRASS.color()
    } else if y < 60 {
        WATER.color()
    } else {
        AIR.color()
    }
}
