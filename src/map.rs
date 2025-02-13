use crate::game::{Game, GAME_INSTANCE};
use crate::player::Player;
use crate::utils::Vector2D;
use crossterm::style::Stylize;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

pub(crate) struct Map {
    player: Arc<Mutex<Player>>,
}

impl Map {
    pub fn new(player: Arc<Mutex<Player>>) -> Map {
        Map { player }
    }

    pub fn draw(&mut self) {
        let mut stdout = stdout();

        let total_lines = Self::SIZE.1 as u16 + 4;
        let mut player = self.player.lock().unwrap();

        execute!(stdout, cursor::MoveUp(total_lines as u16)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        if let Some(body) = player.body.as_mut() {
            println!("Score: {}", body.len() - 1);

            let horizontal_line = "───".repeat(Self::SIZE.0 as usize + 1);
            println!("┌{}┐", horizontal_line);

            for y in 0..=Self::SIZE.1 as u8 {
                print!("│");
                for x in 0..=Self::SIZE.0 as u8 {
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
