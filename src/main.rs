// Just for during development
#![allow(unused)]

use macroquad::{prelude::*, text};
use std::env;
use std::time::{Duration, Instant};
use std::io::Write;

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
    feedback: bool,
}

const WIDTH: i32 = 600;
const HEIGHT: i32 = 200;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;
const FRAME_TIME: u64 = 33;
const GAMEPLAY_FRAME_TIME: u64 = 20;

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
            feedback: false,
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

        self.x += 8 + (self.score / 50);
        self.feedback = false;
        for i in 0..self.obstacles.len() {
            if self.obstacles[i] < self.x {
                self.obstacles[i] = (self.score / 50) * 300 + self.x + WIDTH + (self.rng.read() * 300.0) as i32;
                self.score += 1;
                self.feedback = true;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FeedbackType {
    Visual,
    Audible,
    Tactile,
}

fn deliver_separate_feedback(feedback_type: FeedbackType, sound: &macroquad::audio::Sound) {
    match feedback_type {
        FeedbackType::Audible => {
            macroquad::audio::play_sound_once(sound);
        },
        FeedbackType::Tactile => {},
        FeedbackType::Visual => panic!(
            "The function `deliver_separate_feedback` is only intended for audible or tactile feedback."
        ),
    }
}

async fn gameplay(player_name: &str, args: &Vec<String>, feedback_type: FeedbackType, save_data: bool, sound: &macroquad::audio::Sound) -> i32 {
    // Initialize state
    let mut next_frame_time = Instant::now() + Duration::from_millis(GAMEPLAY_FRAME_TIME);
    let mut state = GameState::init();
    loop {
        let now = Instant::now();
        std::thread::sleep(next_frame_time.saturating_duration_since(now));
        next_frame_time += Duration::from_millis(GAMEPLAY_FRAME_TIME);

        state.update();
        if state.dead {
            if save_data {
                let mut f = std::fs::OpenOptions::new()
                    .append(true)
                    .read(false)
                    .create(true)
                    .open("data.txt")
                    .expect("There was a problem opening the file.");
                writeln!(f, "{{\"player\":\"{}\", \"feedback\":\"{}\", \"score\":{}}}", player_name, args[1], state.score);
            }
            return state.score;
        }

        let width = screen_width();
        let height = screen_height();
        clear_background(DARKGRAY);
        if state.feedback {
            if feedback_type == FeedbackType::Visual {
                clear_background(DARKGREEN);
            } else {
                deliver_separate_feedback(feedback_type, &sound);
            }
        }
        draw_text(player_name, 10.0, 30.0, 40.0, WHITE);
        state.render(width / 2.0 - FWIDTH / 2.0, height / 2.0 - FHEIGHT / 2.0);

        next_frame().await
    }
}

async fn dialogue(player_name: &str, message: &str) {
    // Initialize state
    let mut next_frame_time = Instant::now() + Duration::from_millis(FRAME_TIME);
    loop {
        let now = Instant::now();
        std::thread::sleep(next_frame_time.saturating_duration_since(now));
        next_frame_time += Duration::from_millis(FRAME_TIME);

        let width = screen_width();
        let height = screen_height();
        clear_background(DARKGRAY);

        draw_text(player_name, 10.0, 30.0, 40.0, WHITE);

        let text_size = measure_text(message, None, 30, 1.0);
        draw_text(message, width / 2.0 - text_size.width / 2.0, height / 2.0 + text_size.height / 2.0, 30.0, WHITE);

        let text_size = measure_text("Press space or w to continue.", None, 30, 1.0);
        draw_text("Press space or w to continue.", width / 2.0 - text_size.width / 2.0, height - 20.0, 30.0, WHITE);

        next_frame().await;

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space) {
            break;
        }
    }
}

async fn timer(player_name: &str) {
    // Initialize state
    let mut next_frame_time = Instant::now() + Duration::from_millis(FRAME_TIME);
    let mut timer = 30 * 10;
    loop {
        let now = Instant::now();
        std::thread::sleep(next_frame_time.saturating_duration_since(now));
        next_frame_time += Duration::from_millis(FRAME_TIME);

        let width = screen_width();
        let height = screen_height();
        clear_background(DARKGRAY);

        draw_text(player_name, 10.0, 30.0, 40.0, WHITE);

        let text = format!("{}", (timer + 29) / 30);
        timer -= 1;
        let text_size = measure_text(&text, None, 80, 1.0);
        draw_text(&text, width / 2.0 - text_size.width / 2.0, height / 2.0 + text_size.height / 2.0, 80.0, WHITE);

        next_frame().await;

        if timer <= 0 {
            break;
        }
    }
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
    let player_name: &str = &args[2];
    let sound = macroquad::audio::load_sound("beep.wav").await.expect("Failed to load beep sound!");
    // Introduction
    dialogue(player_name, "Welcome to Noah's science fair experiment.").await;
    dialogue(player_name, "You will be playing a very simple endless runner game.").await;
    dialogue(player_name, &format!("The game will also have {} feedback.", &args[1])).await;
    dialogue(player_name, "You will get 5 rounds: 2 test rounds and 3 practice rounds.").await;
    dialogue(player_name, "Your score on the practice rounds will not be recorded.").await;
    dialogue(player_name, "They will be in the following order:").await;
    dialogue(player_name, "Practice, Test, Practice, Practice, Test").await;
    dialogue(player_name, "You will use either the w or space key to jump.").await;
    dialogue(player_name, "Now, let's begin the experiment.").await;
    // Rounds
    dialogue(player_name, "You will now begin round 1 (a practice round).").await;
    timer(player_name).await;
    let score = gameplay(player_name, &args, feedback, false, &sound).await;
    dialogue(player_name, &format!("You jumped over {} obstacles.", score)).await;

    dialogue(player_name, "You will now begin round 2 (a test round).").await;
    timer(player_name).await;
    let score = gameplay(player_name, &args, feedback, true, &sound).await;
    dialogue(player_name, &format!("You jumped over {} obstacles.", score)).await;

    dialogue(player_name, "You will now begin round 3 (a practice round).").await;
    timer(player_name).await;
    let score = gameplay(player_name, &args, feedback, false, &sound).await;
    dialogue(player_name, &format!("You jumped over {} obstacles.", score)).await;

    dialogue(player_name, "You will now begin round 4 (a practice round).").await;
    timer(player_name).await;
    let score = gameplay(player_name, &args, feedback, false, &sound).await;
    dialogue(player_name, &format!("You jumped over {} obstacles.", score)).await;

    dialogue(player_name, "You will now begin round 5 (a test round).").await;
    timer(player_name).await;
    let score = gameplay(player_name, &args, feedback, true, &sound).await;
    dialogue(player_name, &format!("You jumped over {} obstacles.", score)).await;

    dialogue(player_name, "Thank you for participating! Now go to the next page.").await;
    dialogue(player_name, "You are finished with the experiment.").await;
}