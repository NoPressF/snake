mod game;
mod map;
mod player;
mod utils;

use crossterm::event::{self, KeyCode, KeyEvent};
use game::Game;
use map::Map;
use player::Player;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                let mut player = player_clone.lock().unwrap();
                match code {
                    KeyCode::Up | KeyCode::Char('w') => player.change_direction((0, -1)),
                    KeyCode::Down | KeyCode::Char('s') => player.change_direction((0, 1)),
                    KeyCode::Left | KeyCode::Char('a') => player.change_direction((-1, 0)),
                    KeyCode::Right | KeyCode::Char('d') => player.change_direction((1, 0)),
                    _ => {}
                }
            }
        }
    });

    let mut last_update = Instant::now();

    loop {
        let mut player = player.lock().unwrap();

        if last_update.elapsed() >= Game::INTERVAL {
            player.move_forward();
            last_update = Instant::now();
        }

        Map::draw(&mut player);
        thread::sleep(Duration::from_millis(15));
    }
}
