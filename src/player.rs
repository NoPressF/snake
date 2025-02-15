use crate::game::{Game, GAME_INSTANCE};
use crate::map::Map;
use crate::storage::Storage;
use crate::utils;
use crate::utils::Utils;
use crossterm::style::Color;
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::{Arc, Mutex, MutexGuard};
use utils::Vector2D;

pub struct Player {
    pub body: Option<Vec<Vector2D<i16>>>,
    pub direction: Option<(i8, i8)>,
    pub score: u16,
    pub highest_score: Option<u16>,
}

impl Player {
    pub fn new() -> Player {
        let mut player = Player {
            body: None,
            direction: None,
            score: 0,
            highest_score: Some(0),
        };

        player.body = player.get_center_body_pos();
        player.direction = player.get_random_direction();

        match Storage::load_highest_score() {
            Ok(Some(highest_score)) => {
                player.highest_score = Some(highest_score);
            }
            Ok(None) => {
                let _ = Storage::save_highest_score(&player);
            }
            Err(_) => {}
        }

        player
    }

    pub fn get_random_direction(&self) -> Option<(i8, i8)> {
        let random_direction_index = rand::rng().random_range(0..Self::RANDOM_DIRECTIONS.len());
        Option::from(Self::RANDOM_DIRECTIONS[random_direction_index])
    }

    pub fn set_direction(&mut self, direction: Option<(i8, i8)>) {
        self.direction = direction;
    }

    pub fn get_center_body_pos(&self) -> Option<Vec<Vector2D<i16>>> {
        let mut body: Vec<Vector2D<i16>> = Vec::new();

        for i in 0..Self::SIZE {
            body.push(Vector2D {
                x: (i + (Map::SIZE.0 / 2)) as i16,
                y: (Map::SIZE.1 / 2) as i16,
            });
        }

        Option::from(body)
    }

    pub fn set_body_pos(&mut self, pos: Option<Vec<Vector2D<i16>>>) {
        self.body = pos;
    }

    fn get_new_head(&self) -> Option<Vector2D<i16>> {
        if let Some(body) = &self.body {
            if let Some(head) = body.first() {
                return Some(Vector2D {
                    x: head.x + self.direction.unwrap().0 as i16,
                    y: head.y + self.direction.unwrap().1 as i16,
                });
            }
        }

        None
    }

    pub fn move_forward(&mut self) {
        let mut new_head = self.get_new_head().unwrap();

        if new_head.x < 0 {
            new_head.x = Map::SIZE.0 as i16 + 1;
        } else if new_head.x >= Map::SIZE.0 as i16 + 1 {
            new_head.x = 0;
        }

        if new_head.y < 0 {
            new_head.y = Map::SIZE.1 as i16 - 1;
        } else if new_head.y >= Map::SIZE.1 as i16 + 1 {
            new_head.y = 0;
        }

        if let Some(body) = &self.body {
            if body.contains(&new_head) {
                GAME_INSTANCE.lock().unwrap().restart(self);
                return;
            }
        }

        let game_instance = GAME_INSTANCE.lock().unwrap();

        if let Some(food_pos) = game_instance.get_food_pos() {
            if new_head == food_pos {
                self.pickup_food(game_instance);
            }
        }

        if let Some(body) = self.body.as_mut() {
            body.insert(0, new_head);
            body.pop();
        }
    }

    pub fn pickup_food(&mut self, mut game_instance: MutexGuard<Game>) {
        game_instance.remove_food();
        game_instance.generate_food();
        self.grow();
        self.score += 1;

        if Some(self.score) > self.highest_score {
            self.highest_score = Some(self.score);
            Storage::save_highest_score(self).unwrap();
        }
    }

    pub fn grow(&mut self) {
        if let Some(body) = self.body.as_mut() {
            if let Some(last) = body.last() {
                body.push(last.clone());
            }
        }
    }

    pub fn change_direction(&mut self, new_direction: Option<(i8, i8)>) {
        if let (Some(current), Some(new)) = (self.direction, new_direction) {
            if current.0 + new.0 == 0 && current.1 + new.1 == 0 {
                return;
            }
        }
        self.direction = new_direction;
    }

    pub fn get_body_colors(&mut self) -> Option<Vec<Color>> {
        if let Some(body) = self.body.as_mut() {
            let mut body_colors = Vec::new();
            for index in 0..body.len() {
                let color = Utils::lerp_rgb_color(
                    index as f32 / body.len() as f32,
                    Game::PLAYER_HEAD_COLOR,
                    Game::PLAYER_TAIL_COLOR,
                );
                body_colors.push(color);
            }

            return Some(body_colors);
        }
        None
    }

    const SIZE: u16 = 2;
    const RANDOM_DIRECTIONS: [(i8, i8); 5] = [(0, 1), (-1, 0), (1, 0), (0, -1), (0, 1)];
}

lazy_static! {
    pub static ref PLAYER_INSTANCE: Arc<Mutex<Player>> = Arc::new(Mutex::new(Player::new()));
}
