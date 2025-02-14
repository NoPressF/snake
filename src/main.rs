mod game;
mod map;
mod player;
mod utils;

use crossterm::cursor::Hide;
use crossterm::event::{self, KeyCode, KeyEvent};
use crossterm::execute;
use game::Game;
use map::Map;
use player::Player;
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    execute!(stdout(), Hide).unwrap();

    let player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);
    let mut map = Map::new(player.clone());

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                if let Ok(mut player) = player_clone.lock() {
                    match code {
                        KeyCode::Up | KeyCode::Char('w') => player.change_direction(Some((0, -1))),
                        KeyCode::Down | KeyCode::Char('s') => player.change_direction(Some((0, 1))),
                        KeyCode::Left | KeyCode::Char('a') => player.change_direction(Some((-1, 0))),
                        KeyCode::Right | KeyCode::Char('d') => player.change_direction(Some((1, 0))),
                        _ => {}
                    }
                }
            }
        }
    });

    let mut last_update_snake = Instant::now();

    loop {
        if last_update_snake.elapsed() >= Game::INTERVAL {
            if let Ok(mut player) = player.lock() {
                player.move_forward();
            }
            last_update_snake = Instant::now();
            map.draw();
        }
    }
}