use macroquad::hash;
use macroquad::math::vec2;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Window;
use crate::game_world::GameWorld;

pub fn render_ui (paused: &mut bool, tick: &mut i32, tick_speed: &mut f64, world: &mut GameWorld) -> () {

    world.draw_cells();

    Window::new(hash!(), vec2(10., 10.), vec2(250., 250.))
        .label("Settings").titlebar(true)
        .ui(&mut root_ui(), |ui| {
            if ui.button(None, "Play") {
                *paused = false;
            }
            ui.same_line(0.0); // THIS CONNECTS THE WIDGET ABOVE AND BLOW
            if ui.button(None, "Pause") {
                *paused = true;
            }
            ui.label(None, "Total Ticks: ");
            ui.same_line(0.);
            ui.label(None, &*tick.clone().to_string());
            ui.same_line(0.);
            if ui.button(None, "Step") {
                world.update();
                *tick += 1;
            }
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
            if ui.button(None, "Clear") {
                world.clear();
            }
            ui.same_line(0.);
            if ui.button(None, "Random") {
                world.randomize()
            };
        });
}