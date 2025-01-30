use crate::block::{set_block, AIR};
use colored::ColoredString;

#[derive(Clone)]
pub struct Chunk {
    data: Vec<Vec<Vec<ColoredString>>>,
}

impl Chunk {
    pub fn new(chunk_x: isize, chunk_z: isize, seed: u64) -> Self {
        let mut data = vec![vec![vec![AIR.color(); crate::Z_PER_CHUNK]; crate::MAX_Y]; crate::X_PER_CHUNK];
        for x in 0..crate::X_PER_CHUNK {
            for z in 0..crate::Z_PER_CHUNK {
                for y in 0..crate::MAX_Y {
                    data[x][y][z] = set_block(
                        (x as isize + chunk_x * crate::X_PER_CHUNK as isize) as f64,
                        y,
                        (z as isize + chunk_z * crate::Z_PER_CHUNK as isize) as f64,
                        seed,
                    );
                }
            }
        }
        Chunk { data }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &ColoredString {
        &self.data[x][y][z]
    }

    pub fn _to_string(&self, z: usize) {
        for y in (0..crate::MAX_Y).rev() {
            let mut s = String::new();
            for x in 0..crate::X_PER_CHUNK {
                s.push_str(&self.data[x][y][z].to_string());
            }
            println!("{}", s);
        }
    }
}
