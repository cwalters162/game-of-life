use macroquad::prelude::*;
use macroquad::{main};
use macroquad::prelude::scene::camera_pos;
use crate::ui::render_ui;
mod cell;
mod game_world;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}

#[main(window_conf)]
async fn main() {

    //Initialization of the game state
    let rows = 50;
    let columns= 50;

    let mut previous_time = get_time();
    let mut lag = 0.0;

    let mut world = game_world::GameWorld::new(rows, columns);
    let mut paused = true;

    let mut tick = 0;
    let mut tick_speed = 0.50;

    // let mut offset = (columns as f32 / 2f32 * -0.001, rows as f32 / 2f32 * 0.001);
    let mut offset = (0f32, 0f32);
    let mut zoom = 0.025;

    loop {
        let current_time = get_time();
        let elapsed = current_time - previous_time;
        previous_time = get_time();
        lag += elapsed;
        clear_background(BLACK);

        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
                #[cfg(target_arch = "wasm32")]
                    let y = if y < 0.0 {
                    -1.0
                } else if y > 0.0 {
                    1.0
                } else {
                    0.0
                };
                zoom *= 1.1f32.powf(y);
            }
            _ => (),
        }

        if is_key_down(KeyCode::Left) {
            offset.0 -= 0.01;
        }
        if is_key_down(KeyCode::Right) {
            offset.0 += 0.01;
        }
        if is_key_down(KeyCode::Up) {
            offset.1 += 0.01;
        }
        if is_key_down(KeyCode::Down) {
            offset.1 -= 0.01;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        let cam = Camera2D {
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            offset: vec2(offset.0, offset.1),
            ..Default::default()
        };

        set_camera(&cam);

        //process player input
        world.check_player_draw(&cam);

        //update the game state
        while lag >= tick_speed {

            if !paused {
                world.update();
                tick += 1;
            };

            lag -= tick_speed;
        }
        world.draw_cells();

        set_default_camera();
        //render the UI separately from the game updates.
        render_ui(&mut paused, &mut tick, &mut tick_speed, &mut world);
        draw_text(&*format!("OFFSET: {}, {}", offset.0, offset.1), 0., 32., 32., RED);
        draw_text(&*format!("ZOOM: {}", zoom), 0., 64., 32., RED);
        let screen_to_world = cam.screen_to_world(Vec2::from(mouse_position()));
        draw_text(&*format!("Screen to World: {}", screen_to_world), 0., 96., 32., RED);

        next_frame().await
    }
}

