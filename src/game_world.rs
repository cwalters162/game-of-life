use std::collections::{BTreeMap, HashMap, HashSet};
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::prelude::{Camera2D, get_time, screen_height, screen_width, Vec2, vec2};
use macroquad::rand::{gen_range, rand, srand};
use crate::cell::Cell;

#[derive(Clone)]
pub struct GameWorld {
    pub rows: i32,
    pub cols: i32,
    pub cells: BTreeMap<Pos2d, Cell>
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

    pub fn create_grid(rows: i32, columns: i32) -> BTreeMap<Pos2d, Cell> {
        let mut grid: BTreeMap<Pos2d, Cell> = BTreeMap::new();
        let x_left = columns / 2 * -1;
        let x_right = columns / 2;
        let y_top = rows / 2 * -1;
        let y_bottom = rows / 2;

        for y in y_top..y_bottom{
            for x in x_left..x_right {
                if gen_range(1, 100) > 50 {
                    grid.insert(Pos2d { x, y }, Cell::new(x, y));
                }
            }
        }

        grid
    }

    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.cells.contains_key(&Pos2d { x, y })
    }

    //TODO: """Once Binary Tree is introduced be sure to change this so that
    // if there is a living cell and any of the neighboring "missing cells"
    // has 2 or more living neighbors to then be inserted one all the cells have been checked.
    // which means there will likely be a Vec of positions to insert new living cells at the end of the checks.
    pub fn check_cells(&mut self) {
        let mut pos_to_remove = Vec::<Pos2d>::new();
        let mut new_cells = HashMap::<Pos2d, Cell>::new();
        // Check the current cells neighbors,
        // if it's alive move on
        // if it's dead, check it's neighbors, add to the pos list of checked cells
        // if it's in that list check if it needs to be a new cell or not
        // set current cell dead or alive.
        for (pos, _) in self.cells.clone() {
            //-1, -1 | 0,-1 | 1, -1
            //-1,  0 | 0, 0 | 1,  0
            //-1,  1 | 0, 1 | 1,  1

            let mut num_alive = 0;
            if self.is_alive(pos.x - 1, pos.y - 1) {
                    num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x - 1, y: pos.y - 1}) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x - 1, y: pos.y - 1}, Cell::new(pos.x - 1, pos.y - 1));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x, pos.y - 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x, y: pos.y - 1}) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x, y: pos.y - 1}, Cell::new(pos.x, pos.y - 1));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y - 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x + 1, y: pos.y - 1}) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x + 1, y: pos.y - 1}, Cell::new(pos.x + 1, pos.y - 1));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x - 1, pos.y) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x - 1, y: pos.y }) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x - 1, y: pos.y }, Cell::new(pos.x - 1, pos.y));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x + 1, y: pos.y }) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x + 1, y: pos.y }, Cell::new(pos.x + 1, pos.y));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x - 1, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x - 1, y: pos.y + 1}) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x - 1, y: pos.y + 1}, Cell::new(pos.x - 1, pos.y + 1));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x, y: pos.y + 1 }) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x, y: pos.y + 1 }, Cell::new(pos.x, pos.y + 1));
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {x: pos.x + 1, y: pos.y + 1}) {
                    3 => {
                        new_cells.insert(Pos2d {x: pos.x + 1, y: pos.y + 1}, Cell::new(pos.x + 1, pos.y + 1));
                    }
                    _ => {}
                }
            }
            if num_alive == 2 || num_alive == 3 {

            } else {
                pos_to_remove.push(pos);
            }
        }
        for pos in pos_to_remove {
            self.cells.remove(&pos);
        }
        for (pos, cell) in new_cells {
            self.cells.insert(pos, cell);
        }
    }

    pub fn draw_cells(&mut self) {
        for (_, cell) in self.cells.iter_mut() {
            cell.draw();
        }
    }

    pub fn check_player_draw(&mut self, cam: &Camera2D) {

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let screen_to_world = cam.screen_to_world(Vec2::from(mouse_position()));

            let mut x = screen_to_world.x.floor() as i32;
            let mut y = screen_to_world.y.floor() as i32;

            if cfg!(debug_assertions) {
                println!("Mouse X {} Mouse Y {}", mouse_x, mouse_y);
                println!("Screen_to_world X {} Mouse Y {}", screen_to_world.x, screen_to_world.y);
                println!("x: {}, y: {}", x, y);
            }

            let pos = Pos2d {x, y};

            match self.cells.contains_key(&pos) {
                false => {
                    self.cells.insert(pos, Cell::new(x, y));
                }
                true => {
                    self.cells.remove(&pos);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }
    pub fn randomize(&mut self) {
        self.cells = GameWorld::create_grid(self.rows, self.cols)
    }

    pub fn update(&mut self) {
        self.check_cells();
    }

    fn number_of_living_neighbors(&self, pos: &Pos2d) -> i32 {
        let mut num_alive = 0;
        if self.is_alive(pos.x - 1, pos.y - 1) {
            num_alive += 1;
        }
        if self.is_alive(pos.x, pos.y - 1) {
            num_alive += 1;
        }
        if self.is_alive(pos.x + 1, pos.y - 1) {
            num_alive += 1;
        }
        if self.is_alive(pos.x - 1, pos.y) {
            num_alive += 1;
        }
        if self.is_alive(pos.x + 1, pos.y) {
            num_alive += 1;
        }
        if self.is_alive(pos.x - 1, pos.y + 1) {
            num_alive += 1;
        }
        if self.is_alive(pos.x, pos.y + 1) {
            num_alive += 1;
        }
        if self.is_alive(pos.x + 1, pos.y + 1) {
            num_alive += 1;
        }
        num_alive
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug)]
#[derive(Hash)]
pub struct Pos2d {
    x: i32,
    y: i32,
}