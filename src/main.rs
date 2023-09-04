use crate::world::init_empty_world;
use std::env;

mod world;


fn main() -> () {
    println!("Hello, World!");
    let args: Vec<String> = env::args().collect();
    let height: usize = args[1].parse().unwrap(); // TODO: skip unwraps, handle this properly
    let width: usize = args[2].parse().unwrap();

    init_empty_world(height, width);
}