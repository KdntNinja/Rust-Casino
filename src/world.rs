use crate::chunk::Chunk;
use colored::ColoredString;

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new(chunks_x: usize, chunks_z: usize, seed: u64) -> Self {
        let mut chunks = vec![vec![]; chunks_x];
        for chunk_x in 0..chunks_x {
            let mut row = vec![];
            for chunk_z in 0..chunks_z {
                row.push(Chunk::new(chunk_x as isize, chunk_z as isize, seed));
            }
            chunks[chunk_x] = row;
        }
        World { chunks }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &ColoredString {
        let chunk_x = x / crate::X_PER_CHUNK;
        let chunk_z = z / crate::Z_PER_CHUNK;
        self.chunks[chunk_x][chunk_z].get_block(x % crate::X_PER_CHUNK, y, z % crate::Z_PER_CHUNK)
    }

    pub fn to_string(&self, chunk_x: usize, chunk_z: usize, n_chunks: usize) {
        for z in 0..crate::Z_PER_CHUNK {
            for y in (0..crate::MAX_Y).rev() {
                let mut s = String::new();
                for x in 0..(crate::X_PER_CHUNK * n_chunks) {
                    let global_x = (chunk_x * crate::X_PER_CHUNK + x) % (self.chunks.len() * crate::X_PER_CHUNK);
                    let global_z = (chunk_z * crate::Z_PER_CHUNK + z) % (self.chunks[0].len() * crate::Z_PER_CHUNK);
                    s.push_str(&self.get_block(global_x, y, global_z).to_string());
                }
                println!("{}", s);
            }
            println!("\n");
        }
    }
}
