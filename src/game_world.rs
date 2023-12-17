use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::rand::{gen_range};
use crate::cell::Cell;
use crate::config::{NUM_COLS, NUM_ROWS};
use crate::util::{get_height, get_width};

pub struct GameWorld {
    pub cells: Vec<Vec<Cell>>
}

impl GameWorld {
    pub fn new(rows: &f32, columns: &f32) -> GameWorld {
        let grid = GameWorld::create_grid(&rows, &columns);

        GameWorld { cells: grid }
    }

    pub fn create_grid(float_rows: &f32, float_columns: &f32) -> Vec<Vec<Cell>> {
        let mut cells = Vec::<Vec<Cell>>::new();
        let rows: i32 = float_rows.round() as i32;
        let columns: i32 = float_columns.round() as i32;

        for r in 0..rows{
            cells.push(Vec::<Cell>::new());
            for c in 0..columns {
                cells[r as usize].push(Cell::new(r, c))
            }
        }

        cells
    }

    pub fn is_alive(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
            return 0;
        }

        return if self.cells[x as usize][y as usize].alive {
            1
        } else {
            0
        };
    }

    pub fn check_surrounding(&mut self) {
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
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

    pub fn check_player_draw(&mut self) {

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let x = (mouse_x / get_width()) as i32;
            let y = (mouse_y / get_height()) as i32;
            // dbg!("Mouse X {} Mouse Y {}", mouse_x, mouse_y);
            // dbg!("x: {}, y: {}", x, y);

            if x >= NUM_COLS && y >= NUM_ROWS {
                let not_out_of_bounds_x = x - NUM_COLS;
                let not_out_of_bounds_y = y - NUM_ROWS;
                let mut cell = &mut self.cells[(x - not_out_of_bounds_x - 1) as usize]
                    [(y - not_out_of_bounds_y - 1) as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            } else if y >= NUM_ROWS {
                let not_out_of_bounds = y - NUM_ROWS;
                let mut cell = &mut self.cells[x as usize][(y - not_out_of_bounds - 1) as usize];
                cell.toggle_alive();
                cell.toggle_next_alive();
            } else if x >= NUM_COLS {
                let not_out_of_bounds = x - NUM_COLS;
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
