use crate::player::Player;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};

pub(crate) struct Map {}

impl Map {
    pub fn draw(player: &mut Player) {
        let mut stdout = stdout();

        let total_lines = Self::HEIGHT as u16 + 4;

        execute!(stdout, cursor::MoveUp(total_lines)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        let horizontal_line = " ——".repeat(Self::WIDTH as usize + 1);
        println!("{}", horizontal_line);

        for y in 0..=Self::HEIGHT {
            print!("|");
            for x in 0..=Self::WIDTH {
                if x as i8 == player.pos.x && y as i8 == player.pos.y {
                    print!(" {} ", Player::CHAR);
                } else {
                    print!("   ");
                }
            }
            println!("|");
        }

        println!("{}", horizontal_line);
        println!("X: {} - Y: {}", player.pos.x, player.pos.y);

        stdout.flush().unwrap();
    }

    pub const WIDTH: u8 = 25;
    pub const HEIGHT: u8 = 25;
}
