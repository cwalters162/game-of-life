use macroquad::color::{RED, WHITE};
use macroquad::prelude::{BLACK, Color, draw_rectangle};
use macroquad::shapes::draw_rectangle_lines;

#[derive(Clone)]
pub struct Cell {
    grid_x: i32,
    grid_y: i32,
}

impl Cell {
    pub fn new(grid_x: i32, grid_y: i32) -> Cell {
        Cell {
            grid_x,
            grid_y,
        }
    }
    pub fn draw(&mut self) {
        let scale = 1f32;
        draw_rectangle(
            self.grid_x as f32 * scale,
            self.grid_y as f32 * scale,
            scale,
            scale,
            WHITE,
        );
        draw_rectangle_lines(
            self.grid_x as f32 * scale,
            self.grid_y as f32 * scale,
            scale,
            scale,
            scale / 10.,
            BLACK
        );
    }
}
