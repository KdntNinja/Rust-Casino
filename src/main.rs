use terminal_size::{Width, Height, terminal_size};

// Constants for chunk dimensions and maximum height
const X_PER_CHUNK: usize = 5;
const MAX_Y: usize = 200;
const Z_PER_CHUNK: usize = 5;

// Function to determine the block type based on coordinates and seed
fn set_block(x: f64, y: usize, _z: f64, _seed: u64) -> char {
    let mut block = ' ';
    if y < 150 {
        block = '~'; // Water block
    }
    if y < ((x / 16.0).sin() * 16.0 + 140.0) as usize {
        block = '/'; // Slope block
    }
    if y < ((x / 16.0).sin() * 16.0 + 139.0) as usize {
        block = '#'; // Solid block
    }
    block
}

// Define a Chunk struct to hold 3D data of blocks
#[derive(Clone)]
struct Chunk {
    data: Vec<Vec<Vec<char>>>,
}

impl Chunk {
    // Constructor for Chunk, initializes the data with blocks
    fn new(chunk_x: isize, chunk_z: isize, seed: u64) -> Self {
        let mut data = vec![vec![vec![' '; Z_PER_CHUNK]; MAX_Y]; X_PER_CHUNK];
        for x in 0..X_PER_CHUNK {
            for y in 0..MAX_Y {
                for z in 0..Z_PER_CHUNK {
                    data[x][y][z] = set_block(
                        (x as isize + chunk_x * X_PER_CHUNK as isize) as f64,
                        y,
                        (z as isize + chunk_z * Z_PER_CHUNK as isize) as f64,
                        seed,
                    );
                }
            }
        }
        Chunk { data }
    }

    // Method to get a block at specific coordinates within the chunk
    fn get_block(&self, x: usize, y: usize, z: usize) -> char {
        self.data[x][y][z]
    }

    // Method to print the chunk as a string for a specific z-layer
    fn _to_string(&self, z: usize) {
        for y in (0..MAX_Y).rev() {
            let mut s = String::new();
            for x in 0..X_PER_CHUNK {
                s.push(self.data[x][y][z]);
            }
            println!("{}", s);
        }
    }
}

// Define a World struct to hold multiple chunks
struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    // Constructor for World, initializes the chunks with given dimensions and seed
    fn new(chunks_x: usize, chunks_z: usize, seed: u64) -> Self {
        let mut chunks = vec![vec![]; chunks_x];
        for chunk_x in 0..chunks_x {
            let mut row = vec![];
            for chunk_z in 0..chunks_z {
                row.push(Chunk::new(chunk_x as isize, chunk_z as isize, seed));
            }
            chunks[chunk_x] = row;
        }
        for chunk_x in 0..chunks_x {
            for chunk_z in 0..chunks_z {
                chunks[chunk_x][chunk_z] = Chunk::new(chunk_x as isize, chunk_z as isize, seed);
            }
        }
        World { chunks }
    }

    // Method to get a block at global coordinates within the world
    fn get_block(&self, x: usize, y: usize, z: usize) -> char {
        let chunk_x = x / X_PER_CHUNK;
        let chunk_z = z / Z_PER_CHUNK;
        self.chunks[chunk_x][chunk_z].get_block(x % X_PER_CHUNK, y, z % Z_PER_CHUNK)
    }

    // Method to print the world as a string for a specific chunk and number of chunks
    fn to_string(&self, chunk_x: usize, chunk_z: usize, n_chunks: usize) {
        for z in 0..Z_PER_CHUNK {
            for y in (0..MAX_Y).rev() {
                let mut s = String::new();
                for x in 0..(X_PER_CHUNK * n_chunks) {
                    let global_x = (chunk_x * X_PER_CHUNK + x) % (self.chunks.len() * X_PER_CHUNK);
                    let global_z = (chunk_z * Z_PER_CHUNK + z) % (self.chunks[0].len() * Z_PER_CHUNK);
                    s.push(self.get_block(global_x, y, global_z));
                }
                println!("{}", s);
            }
            println!("\n");
        }
    }
}

// Main function to initialize the world and print it
fn main() {
    // Get the terminal size
    let size = terminal_size();
    let (width, _) = size.unwrap_or((Width(80), Height(24))); // Default to 80x24 if size cannot be determined
    let n_chunks = width.0 as usize / X_PER_CHUNK;

    // Create a new world with the calculated number of chunks
    let world = World::new(n_chunks, 8, 0);
    world.to_string(1, 0, n_chunks); // Print the world starting from chunk (1, 0)
    println!("Done");
    loop {} // Infinite loop to keep the program running
}
