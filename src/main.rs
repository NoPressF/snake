mod game;
mod map;
mod player;
mod utils;

use game::Game;
use map::Map;
use player::Player;

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    ExecutableCommand,
};

use crate::player::MoveDirection;
use std::io::Write;
use std::time::Duration;

fn main() {
    let mut player = Player::new();

    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char('w') => player.move_to(MoveDirection::FORWARD),
                    KeyCode::Char('a') => player.move_to(MoveDirection::LEFT),
                    KeyCode::Char('s') => player.move_to(MoveDirection::BACKWARD),
                    KeyCode::Char('d') => player.move_to(MoveDirection::RIGHT),
                    KeyCode::Esc => {
                        return;
                    }
                    _ => {}
                }
            }
        }

        Map::draw(&mut player);
        Game::update();
    }
}
