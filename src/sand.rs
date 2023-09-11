use crate::cell;
use crate::cell::Cell;
use crate::world;
use crate::world::World;
use rand::Rng;
use raylib::color::Color;
//i have no idea how crates or modules work.
pub fn generate_sand() -> Cell {
    let cell = Cell {
        color: Color::YELLOW,
        density: 2,
        move_speed: 1,
        flammability: 0, // Negative -> puts out fire
        move_func: move_sand as fn(&mut World, usize, usize, &Cell, usize, usize),
    };
    cell
}
fn move_sand(
    world: &mut World,
    world_width: usize,
    world_height: usize,
    cell: &Cell,
    x: usize,
    y: usize,
) {
    for m in 0..cell.move_speed {
        println!("Moving cell");
        if y + 1 < world_height {
            if !world::move_down(world, x, y, cell, world_width, world_height) {
                let mut rng = rand::thread_rng();
                let r: f64 = rng.gen(); // generates a float between 0 and 1
                if r > 0.5 {
                    if !world::move_down_left(world, x, y, cell, world_width, world_height) {
                        world::move_down_right(world, x, y, cell, world_width, world_height);
                    }
                } else {
                    if !world::move_down_right(world, x, y, cell, world_width, world_height) {
                        world::move_down_left(world, x, y, cell, world_width, world_height);
                    }
                }
            }
        }
    }
}
