use macroquad::color::{RED, WHITE};
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::draw_rectangle_lines;

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
        let scale = 1f32;
        if self.alive {
            draw_rectangle(
                self.grid_x as f32 * scale,
                self.grid_y as f32 * scale,
                scale,
                scale,
                WHITE,
            );
            draw_rectangle_lines(self.grid_x as f32 * scale,
                                 self.grid_y as f32 * scale,
                                 scale,
                                 scale,
                                 scale / 10.,
                                 RED)
        }
    }

    pub fn toggle_alive(&mut self) -> () {
        self.alive = !self.alive
    }
    pub fn toggle_next_alive(&mut self) -> () {
        self.next_alive = !self.next_alive
    }
}
