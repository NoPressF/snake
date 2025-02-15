use crate::snake::Snake;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct SnakeData {
    highest_score: u16,
}

pub(crate) struct Storage;
impl Storage {
    pub fn save_highest_score(snake: &Snake) -> io::Result<()> {
        let snake_data = SnakeData {
            highest_score: snake.highest_score,
        };

        let json = serde_json::to_string_pretty(&snake_data)?;
        let mut file = File::create("snake.json")?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn load_highest_score() -> io::Result<u16> {
        let file = File::open("snake.json");

        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;

                let snake_data: SnakeData = serde_json::from_str(&contents)?;
                Ok(snake_data.highest_score)
            }
            Err(_) => Ok(0),
        }
    }
}
