mod game;
mod map;
mod player;
mod utils;

use game::Game;
use map::Map;
use player::Player;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use std::time::Duration;
use crossterm::terminal;

fn main() {
    let mut player = Player::new();

    terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Ok(Event::Key(KeyEvent { code, kind, .. })) = event::read() {
                if kind == event::KeyEventKind::Press {
                    match code {
                        KeyCode::Up | KeyCode::Char('w') => player.change_direction((0, -1)),
                        KeyCode::Down | KeyCode::Char('s') => player.change_direction((0, 1)),
                        KeyCode::Left | KeyCode::Char('a') => player.change_direction((-1, 0)),
                        KeyCode::Right | KeyCode::Char('d') => player.change_direction((1, 0)),
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        player.move_forward();

        Map::draw(&mut player);
        Game::update();
    }

    terminal::disable_raw_mode().unwrap();
}
