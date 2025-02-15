mod game;
mod map;
mod player;
mod utils;

use crate::map::MAP_INSTANCE;
use crate::player::PLAYER_INSTANCE;
use crossterm::cursor::Hide;
use crossterm::execute;
use device_query::{DeviceQuery, DeviceState, Keycode};
use game::Game;
use std::io::stdout;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    execute!(stdout(), Hide).unwrap();

    let player = Arc::clone(&PLAYER_INSTANCE);
    let device_state = DeviceState::new();

    thread::spawn(move || loop {
        let keys = device_state.get_keys();
        let mut player_lock = PLAYER_INSTANCE.lock().unwrap();

        for key in keys {
            match key {
                Keycode::W | Keycode::Up => player_lock.change_direction(Some((0, -1))),
                Keycode::S | Keycode::Down => player_lock.change_direction(Some((0, 1))),
                Keycode::A | Keycode::Left => player_lock.change_direction(Some((-1, 0))),
                Keycode::D | Keycode::Right => player_lock.change_direction(Some((1, 0))),
                Keycode::Escape => {
                    break;
                }
                _ => {}
            }
        }
    });

    let mut last_update_snake = Instant::now();

    loop {
        if last_update_snake.elapsed() >= Game::INTERVAL {
            player.lock().unwrap().move_forward();
            last_update_snake = Instant::now();
            MAP_INSTANCE.lock().unwrap().draw();
        }
    }
}
