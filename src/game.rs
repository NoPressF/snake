use crate::map::Map;
use crate::player::Player;
use crate::utils::Vector2D;
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::Mutex;
use std::time::Duration;

pub struct Game {
    food_pos: Option<Vector2D<i16>>,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game { food_pos: None };
        game.generate_food();
        game
    }

    pub fn get_food_pos(&self) -> Option<Vector2D<i16>> {
        self.food_pos
    }

    pub fn remove_food(&mut self) {
        self.food_pos = None
    }

    pub fn generate_food(&mut self) -> Option<Vector2D<i16>> {
        let x = rand::rng().random_range(0..Map::SIZE.0);
        let y = rand::rng().random_range(0..Map::SIZE.1);

        self.food_pos = Some(Vector2D {
            x: x as i16,
            y: y as i16,
        });

        self.food_pos
    }

    pub fn restart(&mut self, player: &mut Player) {
        self.remove_food();
        self.generate_food();
        player.body.as_mut().unwrap().clear();
        player.set_body_pos(player.get_center_body_pos());
        player.set_direction(player.get_random_direction());
        player.score_history.as_mut().unwrap().insert(player.score);
        player.score = 0;
    }

    pub const INTERVAL: Duration = Duration::from_millis(200);
    pub const APPLE_FOOD: char = '🍎';
    pub const PLAYER: char = '■';
}

lazy_static! {
    pub static ref GAME_INSTANCE: Mutex<Game> = Mutex::new(Game::new());
}
