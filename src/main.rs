mod game;
mod snake;
mod storage;
mod utils;

use crossterm::cursor::Hide;
use crossterm::execute;
use device_query::{DeviceQuery, DeviceState, Keycode};
use game::Game;
use std::io::stdout;
use std::time::Instant;

fn main() {
    execute!(stdout(), Hide).unwrap();

    let mut game = Game::new();
    let device_state = DeviceState::new();
    let mut last_update_snake = Instant::now();

    loop {
        let keys = device_state.get_keys();
        let snake = game.get_snake();

        for key in keys {
            match key {
                Keycode::W | Keycode::Up => snake.change_direction(Some((0, -1))),
                Keycode::S | Keycode::Down => snake.change_direction(Some((0, 1))),
                Keycode::A | Keycode::Left => snake.change_direction(Some((-1, 0))),
                Keycode::D | Keycode::Right => snake.change_direction(Some((1, 0))),
                Keycode::Escape => {
                    break;
                }
                _ => {}
            }
        }

        if last_update_snake.elapsed() >= Game::UPDATE_INTERVAL {
            game.update();
            game.draw();
            last_update_snake = Instant::now();
        }
    }
}
