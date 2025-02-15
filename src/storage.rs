use crate::player::Player;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct PlayerData {
    highest_score: u16,
}

pub(crate) struct Storage;
impl Storage {
    pub fn save_highest_score(player: &Player) -> io::Result<()> {
        if let Some(highest) = player.highest_score {
            let player_data = PlayerData {
                highest_score: highest,
            };

            let json = serde_json::to_string_pretty(&player_data)?;
            let mut file = File::create("player.json")?;
            file.write_all(json.as_bytes())?;

            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No highest score found",
            ))
        }
    }

    pub fn load_highest_score() -> io::Result<Option<u16>> {
        let file = File::open("player.json");

        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;

                let player_data: PlayerData = serde_json::from_str(&contents)?;
                Ok(Some(player_data.highest_score))
            }
            Err(_) => Ok(None),
        }
    }
}
