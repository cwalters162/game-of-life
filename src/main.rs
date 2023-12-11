use macroquad::{main};
use macroquad::prelude::*;
const NUM_ROWS: i32 = 200;
const NUM_COLS: i32 = 200;


fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}


#[main(window_conf)]
async fn main() {
    let cell_height: f32 = screen_width() / NUM_ROWS as f32;
    let cell_width: f32 = screen_height() / NUM_COLS as f32;
    let mut world = GameWorld::new(cell_width, cell_height);
    loop {

        world.check_surrounding();
        clear_background(BLACK);

        for row in world.cells.iter() {
            for cell in row {
                cell.draw()
            }
        }
        next_frame().await
    }
}


struct GameWorld {
    cells: Vec<Vec<Cell>>,
}

impl GameWorld {
    fn new(cell_w: f32, cell_h: f32) -> GameWorld {
        let grid = GameWorld::create_grid(cell_w, cell_h);

        GameWorld {
            cells: grid
        }
    }

    fn create_grid(cell_w: f32, cell_h: f32) -> Vec<Vec<Cell>> {
        let mut cells = Vec::<Vec<Cell>>::new();

        for r in 0..NUM_ROWS {
            cells.push(Vec::<Cell>::new());
            for c in 0..NUM_COLS {
                cells[r as usize].push(Cell::new(r as f32, c as f32, cell_w, cell_h))
            }
        }

        cells
    }

    fn is_alive(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= NUM_ROWS || y < 0 || y >= NUM_COLS {
            return 0
        }

        return if self.cells[x as usize][y as usize].alive {
            1
        } else { 0 }
    }

    fn grid_to_index(&self, x: i32, y: i32) -> usize {
        (x + (y * NUM_COLS)) as usize
    }

    fn check_surrounding(&mut self) {
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