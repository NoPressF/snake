use crate::player::Player;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use crate::utils::Vector2D;

pub(crate) struct Map {}

impl Map {
    pub fn draw(player: &mut Player) {
        let mut stdout = stdout();

        let total_lines = Self::HEIGHT as u16 + 4;

        execute!(stdout, cursor::MoveUp(total_lines)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        println!("X: {} - Y: {} - BodyLen: {}", player.body.get(0).unwrap().x, player.body.get(0).unwrap().y, player.body.len());
        let horizontal_line = " ——".repeat(Self::WIDTH as usize + 1);
        println!("{}", horizontal_line);

        for y in 0..=Self::HEIGHT {
            print!("|");
            for x in 0..=Self::WIDTH {
                if player.body.contains(&(Vector2D{x:x as i8, y:y as i8})) {
                    print!(" {} ", Player::CHAR);
                } else {
                    print!("   ");
                }
            }
            println!("|");
        }

        println!("{}", horizontal_line);

        stdout.flush().unwrap();
    }

    pub const WIDTH: u8 = 25;
    pub const HEIGHT: u8 = 25;
}
