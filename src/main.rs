// Just for during development
#![allow(unused)]

use macroquad::prelude::*;
use std::env;
use std::time::{Duration, Instant};

struct Randomizer(u16);

impl Randomizer {
    fn new() -> Randomizer {
        Randomizer(12345)
    }
    fn read(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(101).wrapping_add(125);
        (self.0 as f32) / 65536.0
    }
}

struct GameState {
    x: i32,
    y: f32,
    vy: f32,
    obstacles: [i32; 2],
    rng: Randomizer,
    dead: bool,
    score: i32,
}

const WIDTH: i32 = 600;
const HEIGHT: i32 = 200;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;
const FRAME_TIME: u64 = 33;

fn draw_player(x: f32, y: f32) {
    draw_rectangle(x - 10.0, y - 40.0, 20.0, 40.0, DARKGRAY);
}

fn draw_obstacle(x: f32, y: f32) {
    draw_rectangle(x - 5.0, y - 20.0, 10.0, 20.0, DARKGRAY);
}

impl GameState {
    fn init() -> GameState {
        GameState {
            x: 0,
            y: 0.0,
            vy: 0.0,
            obstacles: [WIDTH+100, WIDTH+450],
            rng: Randomizer::new(),
            dead: false,
            score: 0,
        }
    }
    fn update(&mut self) {
        self.y += self.vy;
        self.vy -= 0.75;
        if self.y < 0.0 {
            self.y = 0.0;
            self.vy = 0.0;
        }
        if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::W)) && self.y < 1.0 {
            self.vy = 12.0;
        }

        self.x += 5;
        for i in 0..self.obstacles.len() {
            if self.obstacles[i] < self.x {
                self.obstacles[i] = self.x + WIDTH + (self.rng.read() * 100.0) as i32;
                self.score += 1;
            }
        }

        for i in 0..self.obstacles.len() {
            if self.y < 20.0 && self.x > self.obstacles[i] - 15 && self.x < self.obstacles[i] + 15 {
                self.dead = true;
            }
        }
    }
    fn render(&mut self, x: f32, y: f32) {
        // Background
        draw_rectangle(x, y, FWIDTH, FHEIGHT, LIGHTGRAY);
        // Draw player
        draw_player(x + 20.0, y + FHEIGHT - self.y);
        // Draw obstacles
        for i in 0..self.obstacles.len() {
            draw_obstacle((self.obstacles[i] - self.x) as f32 + x + 20.0, y + FHEIGHT);
        }
    }
}

enum FeedbackType {
    Visual,
    Audible,
    Tactile,
}

#[macroquad::main("Feedback Test")]
async fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("You need to have two arguments (feedback type and player name)!");
    }
    let feedback = if args[1] == "visual" {
        FeedbackType::Visual
    } else if args[1] == "audible" {
        FeedbackType::Audible
    } else if args[1] == "tactile" {
        FeedbackType::Tactile
    } else {
        panic!("You need to type either 'visual', 'audible', or 'tactile'.");
    };
    let _player_name: &str = &args[2];
    // Initialize state
    let mut state = GameState::init();
    let mut next_frame_time = Instant::now() + Duration::from_millis(FRAME_TIME);
    // Main loop
    loop {
        let now = Instant::now();
        std::thread::sleep(next_frame_time.saturating_duration_since(now));
        next_frame_time = now + Duration::from_millis(FRAME_TIME);

        state.update();
        if state.dead {
            println!("Your score was {}.", state.score);
            break;
        }

        let width = screen_width();
        let height = screen_height();
        clear_background(DARKGRAY);
        state.render(width / 2.0 - FWIDTH / 2.0, height / 2.0 - FHEIGHT / 2.0);

        next_frame().await
    }
}