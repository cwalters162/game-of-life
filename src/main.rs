mod cell;
mod game_world;
mod config;
mod util;
use macroquad::{main};
use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 1920,
        window_height: 1080,
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
    let mut world = game_world::GameWorld::new();
    let mut paused = false;
    loop {
        if paused == true {
            next_frame().await
        }
        world.check_surrounding();
        world.check_player_draw();
        clear_background(BLACK);


        for row in world.cells.iter_mut() {
            for cell in row {
                cell.draw()
            }
        }

        next_frame().await
    }
}

