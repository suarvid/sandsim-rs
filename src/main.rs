mod cell;
mod sand;
mod world;
use ::core::borrow;
use raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON;
use raylib::prelude::*;
use std::env;
use world::World;

fn main() -> () {
    let height: usize = 720;
    let width: usize = 720;
    let mut world: World = world::init_empty_world(height, width);
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Sand game")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if d.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            let mouse_pos: Vector2 = d.get_mouse_position();
            let mut aligned_x = translate_to_world(mouse_pos.x);
            let mut aligned_y = translate_to_world(mouse_pos.y);
            if aligned_x >= width {
                aligned_x = width - 1;
            }
            if aligned_y >= height {
                aligned_y = height - 1;
            }
            insert_sand(&mut world, aligned_x, aligned_y);
            print_cursor_position(&mut d);
        }
        print_cursor_position(&mut d);
        update_cells(&mut world, width, height);
        d.draw_text("Sand game!", 12, 12, 20, Color::BLACK);
        draw_world(&mut world, height, width, &mut d);
    }
}
fn insert_sand(world: &mut World, x: usize, y: usize) {
    //shouold rly only add new cell if there is no cell there already, or possibly add it on top.
    world[y][x] = Some(sand::generate_sand());
}

fn translate_to_world(num: f32) -> usize {
    //remember to add divide by CELL_RENDER_SIZE once thats introduced.
    return num as usize;
}

fn print_cursor_position(d: &mut RaylibDrawHandle) {
    let mouse_pos: Vector2 = d.get_mouse_position();
    let x = mouse_pos.x;
    let y = mouse_pos.y;
    println!("Mouse Position: {x}, {y}");
}

fn draw_world(world: &mut World, height: usize, width: usize, d: &mut RaylibDrawHandle) {
    d.clear_background(Color::BLACK);

    for y in 0..height {
        for x in 0..width {
            //should use CELL_RENDER_SIZE here also.
            match world[y][x] {
                Some(c) => {
                    //currently not handling different sizes of sand properly.
                    d.draw_rectangle(x as i32, y as i32, 50, 50, c.color);
                }
                _ => {
                    // d.draw_rectangle((1 * j) as i32, (1 * i) as i32, 100, 100, Color::BLACK);
                }
            }
        }
    }
}

fn update_cells(world: &mut World, world_width: usize, world_height: usize) {
    for y in (0..world_height).rev() {
        for x in (0..world_width).rev() {
            match world[y][x] {
                Some(c) => {
                    println!("Moving cell {x} {y}");
                    (c.move_func)(world, world_width, world_height, &c, x, y);
                }
                _ => {
                    //do nothing.
                } // world[row][col]->lifetime > 0 ? world[row][col]->lifetime-- : world[row][col]->lifetime;
                  // if (world[row][col]->lifetime == 0) {
                  //     free(world[row][col]);
                  //     world[row][col] = create_empty_cell(row, col);
                  // }
            }
        }
    }
}
