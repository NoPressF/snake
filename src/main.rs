mod game;
mod map;
mod player;
mod utils;

use crossterm::cursor::Hide;
use crossterm::execute;
use device_query::{DeviceQuery, DeviceState, Keycode};
use game::Game;
use map::Map;
use player::Player;
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    execute!(stdout(), Hide).unwrap();

    let player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);
    let mut map = Map::new(player.clone());
    let device_state = DeviceState::new();

    thread::spawn(move || loop {
        if let Ok(mut player) = player_clone.lock() {
            let keys = device_state.get_keys();

            for key in keys {
                match key {
                    Keycode::W | Keycode::Up => player.change_direction(Some((0, -1))),
                    Keycode::S | Keycode::Down => player.change_direction(Some((0, 1))),
                    Keycode::A | Keycode::Left => player.change_direction(Some((-1, 0))),
                    Keycode::D | Keycode::Right => player.change_direction(Some((1, 0))),
                    Keycode::Escape => {
                        break;
                    }
                    _ => {}
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
