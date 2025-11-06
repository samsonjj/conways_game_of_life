use macroquad::prelude::*;

use crate::GameOfLife;

pub fn ftoi(x: f32, y: f32, game: &GameOfLife) -> (usize, usize) {
    let square_width = screen_width() / (game.width as f32);
    let (a, b) = (y / square_width, x / square_width);
    return (a as usize, b as usize);
}

pub fn draw_hollow_rectangle(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) {
    let m = thickness / 2.;
    draw_line(x, y + m, x + w, y + m, thickness, color);
    draw_line(x, y + h - m, x + w, y + h - m, thickness, color);
    draw_line(x + m, y, x + m, y + h, thickness, color);
    draw_line(x + w - m, y, x + w - m, y + h, thickness, color);
}

pub fn square_width_f(game: &GameOfLife) -> f32 {
    return screen_width() / (game.width as f32);
}
