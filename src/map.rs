use crate::game::{Game, GAME_INSTANCE};
use crate::player::PLAYER_INSTANCE;
use crate::utils::Vector2D;
use crossterm::style::{Color, Stylize};
use crossterm::{cursor, execute, terminal};
use lazy_static::lazy_static;
use std::io::{stdout, Write};
use std::sync::Mutex;

pub struct Map;
impl Map {
    pub fn new() -> Map {
        Map {}
    }

    pub fn draw(&mut self) {
        let mut stdout = stdout();

        let total_lines = Self::SIZE.1 + 4;
        let mut player = PLAYER_INSTANCE.lock().unwrap();
        let body = player.body.clone();

        execute!(stdout, cursor::MoveUp(total_lines)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        if let Some(body) = body {
            if let Some(body_colors) = player.get_body_colors() {
                let game_instance = GAME_INSTANCE.lock().unwrap();
                let mut score = format!("Score: {}", player.score);

                if player.highest_score > 0 {
                    score.push_str(&format!(" - Highest Score: {}", player.highest_score));
                }

                println!("{}", score);

                let horizontal_line = "───".repeat(Self::SIZE.0 as usize + 1);
                println!("┌{}┐", horizontal_line);

                for y in 0..=Self::SIZE.1 {
                    print!("│");
                    for x in 0..=Self::SIZE.0 {
                        let pos = Vector2D {
                            x: x as i16,
                            y: y as i16,
                        };

                        if let Some(body_index) = body.iter().position(|&p| p == pos) {
                            let body_color = body_colors
                                .get(body_index)
                                .unwrap_or(&Game::PLAYER_HEAD_COLOR);

                            let (r, g, b) = match body_color {
                                Color::Rgb { r, g, b } => (*r, *g, *b),
                                _ => panic!("Expected RGB color"),
                            };

                            print!(" {} ", Game::PLAYER.with(Color::Rgb { r, g, b }));
                        } else if game_instance.get_food_pos() == Some(pos) {
                            print!("{} ", game_instance.get_food_char());
                        } else {
                            print!("   ");
                        }
                    }
                    println!("│");
                }

                println!("└{}┘", horizontal_line);
            }
        }

        stdout.flush().unwrap();
    }

    pub const SIZE: (u16, u16) = (20, 20);
}

lazy_static! {
    pub static ref MAP_INSTANCE: Mutex<Map> = Mutex::new(Map::new());
}
