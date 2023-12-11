use macroquad::{main};
use macroquad::prelude::*;
const NUM_ROWS: i32 = 90;
const NUM_COLS: i32 = 160;

#[main("Game of Life")]
async fn main() {
    let cell_height: f32 = screen_width() / NUM_ROWS as f32;
    let cell_width: f32 = screen_height() / NUM_COLS as f32;

    let mut world = GameWorld::new(cell_width, cell_height);
    loop {
        world.check_surrounding();
        clear_background(BLACK);

        for cell in &world.cells {
            cell.draw()
        }

        next_frame().await
    }

    // loop {
    // clear_background(RED);
    //
    // // Get the current screen dimensions
    // let screen_width = screen_width();
    // let screen_height = screen_height();
    //
    // // Define the rectangle dimensions
    // let rect_width = screen_width / 2.0;
    // let rect_height = screen_height / 2.0;
    //
    // // Define the rectangle position
    // let rect_x = (screen_width - rect_width) / 2.0;
    // let rect_y = (screen_height - rect_height) / 2.0;
    //
    // // Draw the rectangle
    // draw_rectangle(rect_x, rect_y, rect_width, rect_height, GREEN);
    //
    // next_frame().await; }
}


struct GameWorld {
    cells: Vec<Cell>,
}

impl GameWorld {
    fn new(cell_w: f32, cell_h: f32) -> GameWorld {
        let grid = GameWorld::create_grid(cell_w, cell_h);

        GameWorld {
            cells: grid
        }
    }

    fn create_grid(cell_w: f32, cell_h: f32) -> Vec<Cell> {
        let mut cells = Vec::<Cell>::new();

        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                cells.push(Cell::new(r as f32, c as f32, cell_w, cell_h))
            }
        }

        cells
    }

    fn is_alive(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
            return 0
        }
        let index = self.grid_to_index(x, y);
        return if self.cells[self.grid_to_index(x, y)].alive {
            1
        } else {
            0
        }
    }

    fn grid_to_index(&self, x: i32, y: i32) -> usize {
        (x + (y * NUM_COLS)) as usize
    }

    fn check_surrounding(&mut self) {
        // Loop over all cells
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // Count the nearby population
                let num_alive = self.is_alive(x - 1, y - 1)
                    + self.is_alive(x, y - 1)
                    + self.is_alive(x + 1, y - 1)
                    + self.is_alive(x - 1, y)
                    + self.is_alive(x + 1, y)
                    + self.is_alive(x - 1, y + 1)
                    + self.is_alive(x, y + 1)
                    + self.is_alive(x + 1, y + 1);

                let center_index = self.grid_to_index(x, y);

                if (num_alive == 2) {
                    // Do nothing
                    self.cells[center_index].next_alive = self.cells[center_index].alive;
                } else if (num_alive == 3) {
                    // Make alive
                    self.cells[center_index].next_alive = true;
                } else {
                    // Make dead
                    self.cells[center_index].next_alive = false;
                }
            }
        }
        // Apply the new state to the cells
        for i in 0..self.cells.len() {
            self.cells[i].alive = self.cells[i].next_alive;
        }
    }
}


struct Cell {
    width: f32,
    height: f32,
    grid_x: f32,
    grid_y: f32,
    alive: bool,
    next_alive: bool,
}

impl Cell {
    fn new(grid_x: f32, grid_y: f32, width: f32, height: f32) -> Cell {
        Cell {
            width,
            height,
            grid_x,
            grid_y,
            alive: rand::gen_range(0, 100) > 50,
            next_alive: false,
        }
    }
    fn draw(&self) {
        if self.alive {
            draw_rectangle(self.grid_x * get_width(), self.grid_y * get_height(),  get_width(), get_height(), WHITE);
        }
    }
}

fn get_height() -> f32 {
    screen_height() / NUM_ROWS as f32
}

fn get_width() -> f32 {
    screen_width() / NUM_COLS as f32
}