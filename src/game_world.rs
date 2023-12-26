use crate::cell::Cell;
use macroquad::prelude::{
    is_mouse_button_down, is_mouse_button_pressed, mouse_position, Camera2D, MouseButton, Vec2,
};
use macroquad::rand::gen_range;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct GameWorld {
    pub rows: f32,
    pub cols: f32,
    pub cells: BTreeMap<Pos2d, Cell>,
    draw_mode: DrawMode,
}

impl GameWorld {
    pub fn new(rows: i32, cols: i32) -> GameWorld {
        let grid = GameWorld::create_grid(rows, cols);

        GameWorld {
            rows: rows as f32,
            cols: cols as f32,
            cells: grid,
            draw_mode: DrawMode::AddCell,
        }
    }

    pub fn create_grid(rows: i32, columns: i32) -> BTreeMap<Pos2d, Cell> {
        let mut grid: BTreeMap<Pos2d, Cell> = BTreeMap::new();
        let x_left = columns / 2 * -1;
        let x_right = columns / 2;
        let y_top = rows / 2 * -1;
        let y_bottom = rows / 2;

        for y in y_top..y_bottom {
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

    pub fn check_cells(&mut self) {
        let mut pos_to_remove = Vec::<Pos2d>::new();
        let mut new_cells = HashMap::<Pos2d, Cell>::new();

        for (pos, _) in self.cells.clone() {
            let mut num_alive = 0;
            if self.is_alive(pos.x - 1, pos.y - 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x - 1,
                    y: pos.y - 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x - 1,
                                y: pos.y - 1,
                            },
                            Cell::new(pos.x - 1, pos.y - 1),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x, pos.y - 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x,
                    y: pos.y - 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x,
                                y: pos.y - 1,
                            },
                            Cell::new(pos.x, pos.y - 1),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y - 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x + 1,
                    y: pos.y - 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x + 1,
                                y: pos.y - 1,
                            },
                            Cell::new(pos.x + 1, pos.y - 1),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x - 1, pos.y) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x - 1,
                    y: pos.y,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x - 1,
                                y: pos.y,
                            },
                            Cell::new(pos.x - 1, pos.y),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x + 1,
                    y: pos.y,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x + 1,
                                y: pos.y,
                            },
                            Cell::new(pos.x + 1, pos.y),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x - 1, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x - 1,
                    y: pos.y + 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x - 1,
                                y: pos.y + 1,
                            },
                            Cell::new(pos.x - 1, pos.y + 1),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x,
                    y: pos.y + 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x,
                                y: pos.y + 1,
                            },
                            Cell::new(pos.x, pos.y + 1),
                        );
                    }
                    _ => {}
                }
            }
            if self.is_alive(pos.x + 1, pos.y + 1) {
                num_alive += 1;
            } else {
                match self.number_of_living_neighbors(&Pos2d {
                    x: pos.x + 1,
                    y: pos.y + 1,
                }) {
                    3 => {
                        new_cells.insert(
                            Pos2d {
                                x: pos.x + 1,
                                y: pos.y + 1,
                            },
                            Cell::new(pos.x + 1, pos.y + 1),
                        );
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
            let screen_to_world = cam.screen_to_world(Vec2::from(mouse_position()));

            let x = screen_to_world.x.floor() as i32;
            let y = screen_to_world.y.floor() as i32;

            let pos = Pos2d { x, y };

            match self.cells.contains_key(&pos) {
                false => {
                    self.draw_mode = DrawMode::AddCell;
                }
                true => self.draw_mode = DrawMode::RemoveCell,
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let screen_to_world = cam.screen_to_world(Vec2::from(mouse_position()));

            let x = screen_to_world.x.floor() as i32;
            let y = screen_to_world.y.floor() as i32;

            if cfg!(debug_assertions) {
                println!("Mouse X {} Mouse Y {}", mouse_x, mouse_y);
                println!(
                    "Screen_to_world X {} Mouse Y {}",
                    screen_to_world.x, screen_to_world.y
                );
                println!("x: {}, y: {}", x, y);
            }

            let pos = Pos2d { x, y };

            match self.draw_mode {
                DrawMode::AddCell => {
                    self.cells.insert(pos, Cell::new(x, y));
                }
                DrawMode::RemoveCell => {
                    self.cells.remove(&pos);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }
    pub fn randomize(&mut self) {
        self.cells = GameWorld::create_grid(self.rows as i32, self.cols as i32)
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

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug, Hash)]
pub struct Pos2d {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum DrawMode {
    AddCell,
    RemoveCell,
}
