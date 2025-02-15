use crate::game::{Game, GAME_INSTANCE};
use crate::player::PLAYER_INSTANCE;
use crate::utils::Vector2D;
use crossterm::style::Stylize;
use crossterm::{cursor, execute, terminal};
use lazy_static::lazy_static;
use std::io::{stdout, Write};
use std::sync::Mutex;

pub struct Map {}

impl Map {
    pub fn new() -> Map {
        Map {}
    }

    pub fn draw(&mut self) {
        let mut stdout = stdout();

        let total_lines = Self::SIZE.1 as u16 + 4;
        let mut player = PLAYER_INSTANCE.lock().unwrap();

        execute!(stdout, cursor::MoveUp(total_lines)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        if let Some(body) = player.body.as_mut() {
            println!("Score: {}", body.len() - 1);

            let horizontal_line = "───".repeat(Self::SIZE.0 as usize + 1);
            println!("┌{}┐", horizontal_line);

            for y in 0..=Self::SIZE.1 {
                print!("│");
                for x in 0..=Self::SIZE.0 {
                    if body.contains(
                        &(Vector2D {
                            x: x as i8,
                            y: y as i8,
                        }),
                    ) {
                        print!(" {} ", Game::PLAYER.green());
                    } else if GAME_INSTANCE.lock().unwrap().get_food_pos()
                        == Some(Vector2D {
                            x: x as i8,
                            y: y as i8,
                        })
                    {
                        print!("{} ", Game::APPLE_FOOD);
                    } else {
                        print!("   ");
                    }
                }
                println!("│");
            }

            println!("└{}┘", horizontal_line);
        }

        stdout.flush().unwrap();
    }

    pub const SIZE: (u8, u8) = (25, 25);
}

lazy_static! {
    pub static ref MAP_INSTANCE: Mutex<Map> = Mutex::new(Map::new());
}
