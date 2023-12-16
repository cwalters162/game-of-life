use macroquad::prelude::*;
use macroquad::{hash, main};
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Group, Window};
use crate::config::NUM_ROWS;

mod cell;
mod config;
mod game_world;
mod util;

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
    let mut paused = true;

    loop {
        clear_background(BLACK);


        world.check_player_draw();

        if !paused {

            world.check_surrounding();

        };

        for row in world.cells.iter_mut() {
            for cell in row {
                cell.draw();
            }
        }


        Window::new(hash!(), vec2(10., 10.), vec2(200., 200.))
            .label("Settings").titlebar(true)
            .ui(&mut root_ui(), |ui| {
                ui.label(vec2(10., 10.),"Testing");
                ui.label(None, "Some random text");
                if ui.button(None, "Pause/Play") {
                    paused = !paused;
                }
                ui.slider(hash!(), "COLUMNS", 0f32..255f32, &mut 0.);
                ui.slider(hash!(), "ROWS", 0f32..255f32, &mut 0.);

                ui.separator();
            });

        next_frame().await
    }
}