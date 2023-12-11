use macroquad::prelude::{screen_height, screen_width};
use crate::config::{NUM_COLS, NUM_ROWS};

pub fn get_height() -> f32 {
    screen_height() / NUM_ROWS as f32
}

pub fn get_width() -> f32 {
    screen_width() / NUM_COLS as f32
}