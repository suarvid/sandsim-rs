use core::num;

use crate::cell;
use crate::cell::Cell;
pub type World = Vec<Vec<Option<cell::Cell>>>;
// pub const WORLD_SIZE: i32 = 512;
// pub const WORLD_RENDER_SIZE: i32 = 1024;
// pub const CELL_RENDER_SIZE: i32 = 2;

//from arvid // NOTE: World should always be indexed world[y][x],
// or (equally) world[row][col], NOT world[x][y]
pub fn init_empty_world(height: usize, width: usize) -> World {
    let mut world: World = Vec::with_capacity(height);
    for row in 0..height {
        world.push(Vec::with_capacity(height));
        for _ in 0..width {
            world[row].push(None);
        }
    }

    world
}
pub fn empty_above(
    world: &mut World,
    x: usize,
    y: usize,
    world_width: usize,
    world_height: usize,
) -> bool {
    if within_world_bounds(y - 1, x, world_height, world_width) {
        match &world[y - 1][x] {
            None => return true,
            _ => return false,
        }
    }
    return false;
}

pub fn empty_below(
    world: &mut World,
    x: usize,
    y: usize,
    world_width: usize,
    world_height: usize,
) -> bool {
    if within_world_bounds(y + 1, x, world_height, world_width) {
        match &world[y + 1][x] {
            None => return true,
            _ => return false,
        }
    }
    return false;
}
pub fn empty_left(
    world: &mut World,
    x: usize,
    y: usize,
    world_width: usize,
    world_height: usize,
) -> bool {
    if within_world_bounds(y, x - 1, world_height, world_width) {
        match &world[y][x - 1] {
            None => return true,
            _ => return false,
        };
    };
    return false;
}

pub fn empty_right(
    world: &mut World,
    x: usize,
    y: usize,
    world_width: usize,
    world_height: usize,
) -> bool {
    if within_world_bounds(y, x + 1, world_height, world_width) {
        match &world[y][x + 1] {
            None => return true,
            _ => return false,
        };
    };
    return false;
}
pub fn within_world_bounds(y: usize, x: usize, world_height: usize, world_width: usize) -> bool {
    return ((y >= 0) && (y < world_height) && (x >= 0) && (x < world_width));
}
//handles density case.
pub fn move_down(
    world: &mut World,
    x: usize,
    y: usize,
    cell: &Cell,
    world_width: usize,
    world_height: usize,
) -> bool {
    if !within_world_bounds(y + 1, x, world_height, world_width) {
        return false;
    }
    match world[y + 1][x] {
        Some(down) => {
            if down.density < cell.density {
                world[y + 1][x] = Some(*cell);
                world[y][x] = Some(down);
                return true;
            }
            return false;
        }
        _ => {
            world[y][x] = None;
            world[y + 1][x] = Some(*cell);
            return true;
        }
    }
}
pub fn move_down_left(
    world: &mut World,
    x: usize,
    y: usize,
    cell: &Cell,
    world_width: usize,
    world_height: usize,
) -> bool {
    if x < 1 || !within_world_bounds(y + 1, x, world_height, world_width) {
        return false;
    }
    match world[y + 1][x - 1] {
        Some(down_left) => {
            if down_left.density < cell.density {
                world[y + 1][x - 1] = Some(*cell);
                world[y][x] = None;
                return true;
            }
            return false;
        }
        _ => {
            world[y][x] = None;
            world[y + 1][x - 1] = Some(*cell);
            return true;
        }
    }
}

pub fn move_down_right(
    world: &mut World,
    x: usize,
    y: usize,
    cell: &Cell,
    world_width: usize,
    world_height: usize,
) -> bool {
    if !within_world_bounds(y + 1, x + 1, world_height, world_width) {
        return false;
    }
    match world[y + 1][x + 1] {
        Some(down_left) => {
            if down_left.density < cell.density {
                world[y + 1][x + 1] = Some(*cell);
                world[y][x] = Some(down_left);
                return true;
            }
            return false;
        }
        _ => {
            world[y][x] = None;
            world[y + 1][x + 1] = Some(*cell);
            return true;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_init_empty_world_is_empty() {
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
