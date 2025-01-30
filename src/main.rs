mod block;
mod chunk;
mod world;

use terminal_size::{Width, Height, terminal_size};
use world::World;

// Constants for chunk dimensions and maximum height
const X_PER_CHUNK: usize = 5;
const MAX_Y: usize = 200;
const Z_PER_CHUNK: usize = 5;

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
}
