use crate::world::World;
use raylib::color::Color;
#[derive(Copy, Clone)]
pub struct Cell {
    pub color: Color,
    pub density: usize,
    pub move_speed: usize,
    pub flammability: i32, // Negative -> puts out fire
    pub move_func: fn(&mut World,usize,usize, &Cell, usize, usize),
}
