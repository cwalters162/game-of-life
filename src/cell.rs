use crate::util::{get_height, get_width};
use macroquad::color::WHITE;
use macroquad::prelude::draw_rectangle;

pub struct Cell {
    grid_x: i32,
    grid_y: i32,
    pub alive: bool,
    pub next_alive: bool,
}

impl Cell {
    pub fn new(grid_x: i32, grid_y: i32) -> Cell {
        Cell {
            grid_x,
            grid_y,
            alive: macroquad::rand::gen_range(0, 100) > 50,
            next_alive: false,
        }
    }
    pub fn draw(&mut self) {
        if self.alive {
            draw_rectangle(
                self.grid_x as f32 * get_width(),
                self.grid_y as f32 * get_height(),
                get_width(),
                get_height(),
                WHITE,
            );
        }
    }

    pub fn toggle_alive(&mut self) -> () {
        self.alive = !self.alive
    }
    pub fn toggle_next_alive(&mut self) -> () {
        self.next_alive = !self.next_alive
    }
}
