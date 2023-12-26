use crate::game_world::GameWorld;
use macroquad::hash;
use macroquad::math::vec2;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Window;

pub fn render_ui(
    paused: &mut bool,
    tick: &mut i32,
    tick_speed: &mut f64,
    world: &mut GameWorld,
) -> () {
    Window::new(hash!(), vec2(0., 400.), vec2(250., 250.))
        .label("Settings")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            if ui.button(None, "Play") {
                *paused = false;
            }
            ui.same_line(0.0); // THIS CONNECTS THE WIDGET ABOVE AND BLOW
            if ui.button(None, "Pause") {
                *paused = true;
            }
            ui.same_line(0.0);
            if ui.button(None, "Step") {
                world.update();
                *tick += 1;
            }

            ui.label(None, "Total Ticks: ");
            ui.same_line(0.);
            ui.label(None, &*tick.clone().to_string());

            ui.label(None, "Total Living: ");
            ui.same_line(0.);
            ui.label(None, &world.cells.len().to_string());

            if ui.button(None, "Slowest") {
                *tick_speed = 1.0;
            }
            ui.same_line(0.0);
            ui.same_line(0.0);
            if ui.button(None, "Slow") {
                *tick_speed = 0.50;
            }
            ui.same_line(0.0);
            if ui.button(None, "Normal") {
                *tick_speed = 0.1;
            }
            ui.same_line(0.0);
            if ui.button(None, "Fast") {
                *tick_speed = 0.05;
            }
            ui.same_line(0.0);
            if ui.button(None, "Fastest") {
                *tick_speed = 0.01;
            }
            ui.slider(hash!(), "Rows", 1f32..250f32, &mut world.rows);
            ui.slider(hash!(), "Columns", 1f32..250f32, &mut world.cols);
            if ui.button(None, "Clear") {
                *tick = 0;
                world.clear();
            }
            ui.same_line(0.);
            if ui.button(None, "Random") {
                *tick = 0;
                world.randomize()
            };
        });
}
