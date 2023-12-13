use crate::util::{get_height, get_width};
use macroquad::color::WHITE;
use macroquad::prelude::draw_rectangle;

pub struct Cell {
    grid_x: f32,
    grid_y: f32,
    pub alive: bool,
    pub next_alive: bool,
}

impl Cell {
    pub fn new(grid_x: f32, grid_y: f32) -> Cell {
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
                self.grid_x * get_width(),
                self.grid_y * get_height(),
                get_width(),
                get_height(),
                WHITE,
            );
        }
    }
}
