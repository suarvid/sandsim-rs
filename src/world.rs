use raylib::color::Color;

pub struct Cell {
    color: Color,
    density: usize,
    move_speed: usize,
    flammability: i32, // Negative -> puts out fire
    move_func: fn(World, usize, usize) -> Coordinates,
}

pub type Coordinates = (usize, usize);
pub type World = Vec<Vec<Option<Cell>>>;

pub fn init_empty_world(height: usize, width: usize) -> World {
    let mut world: Vec<Vec<Option<Cell>>> = Vec::with_capacity(height);

    for row in 0..height {
        world[row] = Vec::with_capacity(width);
        for col in 0..width {
            world[row][col] = None;
        }
    }

    world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_empty_world_is_empty() {
        let height = 100;
        let width = 100;
        let world = init_empty_world(height, width);
        for row in 0..height {
            for col in 0..width {
                let c = &world[row][col];
                assert!(c.is_none());
            }
        }
    }
}
