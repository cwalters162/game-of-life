use crate::config::{NUM_COLS, NUM_ROWS};
use crate::cell::Cell;

pub struct GameWorld {
    pub cells: Vec<Vec<Cell>>,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        let grid = GameWorld::create_grid();

        GameWorld {
            cells: grid
        }
    }

    pub fn create_grid() -> Vec<Vec<Cell>> {
        let mut cells = Vec::<Vec<Cell>>::new();

        for r in 0..NUM_ROWS {
            cells.push(Vec::<Cell>::new());
            for c in 0..NUM_COLS {
                cells[r as usize].push(Cell::new(r as f32, c as f32))
            }
        }

        cells
    }

    pub fn is_alive(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
            return 0
        }

        return if self.cells[x as usize][y as usize].alive {
            1
        } else { 0 }
    }

    // fn grid_to_index(&self, x: i32, y: i32) -> usize {
    //     (x + (y * NUM_COLS)) as usize
    // }

    pub fn check_surrounding(&mut self) {
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
                    continue
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
                    self.cells[x as usize][y as usize].next_alive = self.cells[x as usize][y as usize].alive;
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
}