use macroquad::prelude::*;
use macroquad::{main};
use crate::ui::render_ui;

mod cell;
mod config;
mod game_world;
mod util;
mod ui;

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

    //Initialization of the game state
    let rows: f32 = 200.;
    let columns: f32 = 200.;

    let mut previous_time = get_time();
    let mut lag = 0.0;

    let mut world = game_world::GameWorld::new(&rows, &columns);
    let mut paused = true;

    let mut tick = 0;
    let mut tick_speed = 0.50;

    loop {
        let current_time = get_time();
        let elapsed = current_time - previous_time;
        previous_time = get_time();
        lag += elapsed;
        clear_background(BLACK);

        //process player input
        world.check_player_draw();

        //update the game state
        while lag >= tick_speed {

            if !paused {
                world.check_surrounding();
                tick += 1;
            };

            lag -= tick_speed;
        }

        //render the UI separately from the game updates.
        render_ui(&mut paused, &tick, &mut tick_speed, &mut world);

        next_frame().await
    }
}

