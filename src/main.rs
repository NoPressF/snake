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

    let device_state = DeviceState::new();
    let mut game = Game::new();

    // let mut last_update_main_menu = Instant::now();
    //
    // 'main: loop {
    //     let keys = device_state.get_keys();
    //     for key in keys {
    //         match key {
    //             Keycode::Key1 => {
    //                 break 'main;
    //             }
    //             Keycode::Key2 => {
    //                 game.toggle_wall_collision(!game.get_wall_collision());
    //             }
    //             Keycode::Escape => {
    //                 break 'main;
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     if last_update_main_menu.elapsed() >= Game::UPDATE_MAIN_MENU_INTERVAL {
    //         game.draw_main_menu();
    //         last_update_main_menu = Instant::now();
    //     }
    // }

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
                    return;
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
