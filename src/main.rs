mod game;
mod map;
mod player;
mod utils;

use std::error::Error;
use crossterm::cursor::Hide;
use crossterm::execute;
use game::Game;
use map::Map;
use player::Player;
use std::io::stdout;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;
use winit::event::{DeviceEvent, DeviceId, Ime, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder};
use winit::keyboard::{Key, ModifiersState};


fn main() -> Result<(), Box<dyn Error>> {
    execute!(stdout(), Hide).unwrap();

    let player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);
    let mut map = Map::new(player.clone());

    let event_loop = EventLoop::new();
    let (sender, receiver) = mpsc::channel();

    #[cfg(not(web_platform))]
    {
        let event_loop_proxy = event_loop.unwrap().create_proxy();
        let sender = sender.clone();
        thread::spawn(move || {
            let event_loop = EventLoopBuilder::<MyEventEnum>::with_user_event().build();
            let window = WindowBuilder::new().build(&event_loop).unwrap();

            some_channel.send(window);

            event_loop.run(move |event, _, control_flow| {
                ...
            })
        });
    }


    thread::spawn(move || loop {

    });

        // if event::poll(Duration::from_millis(10)).unwrap() {
        //     if let Ok(character) = stdout.read_key() {
        //         if let Ok(mut player) = player_clone.lock() {
        //             match character {
        //                 console::Key::ArrowUp | console::Key::Char('w') => player.change_direction(Some((0, -1))),
        //                 console::Key::ArrowDown | console::Key::Char('s') => player.change_direction(Some((0, 1))),
        //                 console::Key::ArrowLeft | console::Key::Char('a') => player.change_direction(Some((-1, 0))),
        //                 console::Key::ArrowRight | console::Key::Char('d') => player.change_direction(Some((1, 0))),
        //                 _ => {}
        //             }
        //         }
        //     }
        // }
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