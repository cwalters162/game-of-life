use macroquad::camera::Camera;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::prelude::{Camera2D, screen_height, screen_width, Vec2};
use macroquad::prelude::scene::camera_pos;
use macroquad::rand::{gen_range};
use crate::cell::Cell;

pub struct GameWorld {
    pub rows: i32,
    pub cols: i32,
    pub cells: Vec<Vec<Cell>>
}

impl GameWorld {
    pub fn new(rows: i32, cols: i32) -> GameWorld {
        let grid = GameWorld::create_grid(rows, cols);

        GameWorld {
            rows,
            cols,
            cells: grid
        }
    }

    pub fn create_grid(rows: i32, columns: i32) -> Vec<Vec<Cell>> {
        let mut cells = Vec::<Vec<Cell>>::new();

        for r in 0..rows{
            cells.push(Vec::<Cell>::new());
            for c in 0..columns {
                cells[r as usize].push(Cell::new(r, c))
            }
        }

        cells
    }

    pub fn is_alive(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= self.rows || y < 0 || y >= self.cols {
            return 0;
        }

        return if self.cells[x as usize][y as usize].alive {
            1
        } else {
            0
        };
    }

    pub fn check_surrounding(&mut self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                if x < 0 || x >= self.rows || y < 0 || y >= self.cols {
                    continue;
                }

                let num_alive = self.is_alive(x - 1, y - 1)
                    + self.is_alive(x, y - 1)
                    + self.is_alive(x + 1, y - 1)
                    + self.is_alive(x - 1, y)
                    + self.is_alive(x + 1, y)
                    + self.is_alive(x - 1, y + 1)
                    + self.is_alive(x, y + 1)
                    + self.is_alive(x + 1, y + 1);

                if num_alive == 2 {
                    self.cells[x as usize][y as usize].next_alive =
                        self.cells[x as usize][y as usize].alive;
                } else if num_alive == 3 {
                    self.cells[x as usize][y as usize].next_alive = true;
                } else {
                    self.cells[x as usize][y as usize].next_alive = false;
                }
            }
        }
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                self.cells[r][c].alive = self.cells[r][c].next_alive;
            }
        }
    }

    pub fn draw_cells(&mut self) {
        for row in self.cells.iter_mut() {
            for mut cell in row {
                cell.draw();
            }
        }
    }

    pub fn check_player_draw(&mut self, cam: &Camera2D) {

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let screen_to_world = cam.screen_to_world(Vec2::from(mouse_position()));

            let x = screen_to_world.x as i32;
            let y = screen_to_world.y as i32;
            dbg!("Mouse X {} Mouse Y {}", mouse_x, mouse_y);
            dbg!("Adjusted X {} Adjusted Y {}", mouse_x, mouse_y);
            dbg!("x: {}, y: {}", x, y);

            if x >= self.cols && y >= self.rows {
                let not_out_of_bounds_x = x - self.cols;
                let not_out_of_bounds_y = y - self.rows;
                let mut cell = &mut self.cells[(x - not_out_of_bounds_x - 1) as usize]
                    [(y - not_out_of_bounds_y - 1) as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            } else if y >= self.rows {
                let not_out_of_bounds = y - self.rows;
                let mut cell = &mut self.cells[x as usize][(y - not_out_of_bounds - 1) as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            } else if x >= self.cols {
                let not_out_of_bounds = x - self.cols;
                let mut cell = &mut self.cells[(x - not_out_of_bounds - 1) as usize][y as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            } else {
                let mut cell = &mut self.cells[x as usize][y as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            }
        }
    }

    pub fn clear(&mut self) {
        for row in self.cells.iter_mut() {
            for mut cell in row {
                cell.alive = false;
            }
        }
    }
    pub fn randomize(&mut self) {
        for row in self.cells.iter_mut() {
            for mut cell in row {
                cell.alive = gen_range(0, 10) >= 5;
            }
        }
    }

    pub fn update(&mut self) {
        self.check_surrounding();
    }
}
