use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::render::{draw_hollow_rectangle, ftoi, square_width_f};

mod render;

const GAME_AREA_WIDTH: f32 = 720.;
const NAV_HEIGHT: f32 = 30.;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Alive,
    Dead,
}

struct GameOfLife {
    grid: Vec<Vec<State>>,
    history: VecDeque<Vec<Vec<State>>>,
    width: usize,
    height: usize,
    paused: bool,
    exit: bool, // game loop will terminate when true
    frame: usize,
    speed: usize,
}

const HISTORY_LEN: usize = 10;

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![State::Dead; width]; height];
        Self {
            grid,
            width,
            height,
            history: VecDeque::new(),
            paused: true, // game starts in a paused state
            exit: false,
            frame: 0,
            speed: 1,
        }
    }

    pub fn step(&mut self) -> () {
        // update history
        self.history.push_back(self.grid.clone());
        if self.history.len() > self::HISTORY_LEN {
            self.history.pop_front();
        }

        // update grid
        let mut next_grid = vec![vec![State::Dead; self.width]; self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let num_neighbors = self.count_neighbors(i, j);
                let next_state = match (self.grid[i][j], num_neighbors) {
                    (State::Alive, x) if x < 2 => State::Dead, // underpopulation
                    (State::Alive, x) if x > 3 => State::Dead, // overpopulation
                    (State::Alive, _) => State::Alive,         // overpopulation
                    (State::Dead, x) if x == 3 => State::Alive, // reproduction
                    (State::Dead, _) => State::Dead,
                };
                next_grid[i][j] = next_state;
            }
        }
        self.grid = next_grid;
    }

    pub fn step_back(&mut self) {
        if let Some(previous_grid) = self.history.pop_back() {
            self.grid = previous_grid;
        }
    }

    fn count_neighbors(&self, a: usize, b: usize) -> usize {
        let (a, b) = (a as i32, b as i32);
        return [
            self.geti(a - 1, b),
            self.geti(a - 1, b - 1),
            self.geti(a - 1, b + 1),
            self.geti(a + 1, b),
            self.geti(a + 1, b - 1),
            self.geti(a + 1, b + 1),
            self.geti(a, b - 1),
            self.geti(a, b + 1),
        ]
        .iter()
        .filter(|x| **x == State::Alive)
        .count();
    }

    fn geti(&self, a: i32, b: i32) -> State {
        if a < 0 {
            return State::Dead;
        }
        if b < 0 {
            return State::Dead;
        }
        return self
            .grid
            .get(a as usize)
            .map(|row| row.get(b as usize))
            .flatten()
            .map(|item| *item)
            .unwrap_or(State::Dead);
    }

    pub fn set_size(&mut self, size: usize) {
        self.width = size;
        self.height = size;
        self.grid = vec![vec![State::Dead; size]; size];
    }

    pub fn slow_down(&mut self) {
        self.speed /= 2;
        if self.speed == 0 {
            self.speed = 1;
            self.paused = true;
        }
    }

    pub fn speed_up(&mut self) {
        self.speed *= 2;
    }
}

fn render(game: &GameOfLife) {
    let square_width = screen_width() / (game.width as f32);
    for i in 0..game.height {
        for j in 0..game.width {
            if game.grid[i][j] == State::Alive {
                let x = square_width * (j as f32);
                let y = square_width * (i as f32);
                draw_rectangle(x, y, square_width, square_width, WHITE);
            }
        }
    }
    // render grid lines
    for i in 0..game.height {
        let offset = square_width * i as f32;
        draw_line(0., offset, screen_width(), offset, 1., GRAY);
    }
    for j in 0..game.width {
        let offset = square_width * j as f32;
        draw_line(offset, 0., offset, screen_height(), 1., GRAY);
    }
}

fn render_nav(game: &GameOfLife) {
    let dy = GAME_AREA_WIDTH;
    let text_line_margin = 7.;
    let text_dy = NAV_HEIGHT + dy - text_line_margin;
    draw_rectangle(0., dy, GAME_AREA_WIDTH, NAV_HEIGHT, DARKGRAY);
    let speed_text = format!("{}x", game.speed);
    draw_text(speed_text.as_str(), 20.0, text_dy, 30.0, BLACK);
}

fn mouse_controller(game: &mut GameOfLife) {
    let (x, y) = mouse_position();
    let (a, b) = ftoi(x, y, game);

    // square hover highlight
    if a < game.width && b < game.height {
        let square_width = square_width_f(game);
        let dx = b as f32 * square_width;
        let dy = a as f32 * square_width;
        draw_hollow_rectangle(dx, dy, square_width, square_width, 5.0, WHITE);
    }

    if a >= game.height || b >= game.width {
        return;
    }
    if is_mouse_button_down(MouseButton::Left) {
        game.grid[a][b] = State::Alive;
    }
    if is_mouse_button_down(MouseButton::Right) {
        game.grid[a][b] = State::Dead;
    }
}

fn exit_controller(game: &mut GameOfLife) {
    if is_key_pressed(KeyCode::Escape) {
        game.exit = true;
    }
}

fn step_controller(game: &mut GameOfLife) {
    if is_key_pressed(KeyCode::Space) {
        game.paused = !game.paused;
    }

    if is_key_pressed(KeyCode::U) {
        game.slow_down();
    }
    if is_key_pressed(KeyCode::O) {
        game.speed_up();
    }

    if game.paused {
        if is_key_pressed(KeyCode::L) {
            game.step();
        }
        if is_key_pressed(KeyCode::J) {
            game.step_back();
        }
    } else {
        let frames_per_step = 60 / game.speed;
        if game.frame % frames_per_step == frames_per_step - 1 {
            game.step();
        }
    }
}

fn nav_button(text: &str, dx: f32, game: &mut GameOfLife, f: impl FnOnce(&mut GameOfLife) -> ()) {
    let dy = GAME_AREA_WIDTH;
    let text_line_margin = 7.;
    let text_dy = NAV_HEIGHT + dy - text_line_margin;

    let dims = draw_text(text, dx, text_dy, 30.0, BLACK);
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        if x >= dx
            && x < dx + dims.width
            && y >= text_dy - NAV_HEIGHT + text_line_margin * 2.
            && y < text_dy
        {
            f(game);
        }
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    request_new_screen_size(GAME_AREA_WIDTH, GAME_AREA_WIDTH + NAV_HEIGHT);
    let mut game = GameOfLife::new(20, 20);

    game.grid[12][10] = State::Alive;
    game.grid[10][11] = State::Alive;
    game.grid[11][11] = State::Alive;
    game.grid[11][12] = State::Alive;
    game.grid[12][12] = State::Alive;

    loop {
        if game.exit {
            break;
        }

        // render
        clear_background(BLACK);
        render(&game);
        render_nav(&game);

        nav_button("+", 70., &mut game, |game| game.set_size(game.width + 1));
        nav_button("-", 90., &mut game, |game| game.set_size(game.width - 1));
        nav_button("<<", 120., &mut game, |game| game.slow_down());
        nav_button(">>", 150., &mut game, |game| game.speed_up());
        nav_button(
            match game.paused {
                true => "pause",
                false => "play",
            },
            screen_width() - 75.,
            &mut game,
            |game| game.paused = !game.paused,
        );

        // update
        step_controller(&mut game);
        mouse_controller(&mut game);
        exit_controller(&mut game);
        game.frame += 1;

        // next
        next_frame().await
    }
}
