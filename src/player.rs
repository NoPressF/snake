use crate::game::GAME_INSTANCE;
use crate::map::Map;
use crate::utils;
use rand::Rng;
use utils::Vector2D;

pub struct Player {
    pub body: Option<Vec<Vector2D>>,
    pub direction: Option<(i8, i8)>,
}

impl Player {
    pub fn new() -> Player {
        let mut player = Player {
            body: None,
            direction: None,
        };

        player.body = player.get_center_body_pos();
        player.direction = player.get_random_direction();

        player
    }

    pub fn get_random_direction(&self) -> Option<(i8, i8)> {
        let random_directions: Vec<(i8, i8)> = vec![(0, 1), (-1, 0), (1, 0), (0, -1), (0, 1)];
        let random_direction_index = rand::rng().random_range(0..random_directions.len());
        Option::from(random_directions[random_direction_index])
    }

    pub fn set_direction(&mut self, direction: Option<(i8, i8)>) {
        self.direction = direction;
    }

    pub fn get_center_body_pos(&self) -> Option<Vec<Vector2D>> {
        let mut body = Vec::new();

        for i in 0..Self::SIZE {
            body.push(Vector2D {
                x: i as i8 + (Map::SIZE.0 / 2) as i8,
                y: (Map::SIZE.1 / 2) as i8,
            });
        }

        Option::from(body)
    }

    pub fn set_body_pos(&mut self, pos: Option<Vec<Vector2D>>) {
        self.body = pos;
    }

    fn get_new_head(&self) -> Option<Vector2D> {
        if let Some(body) = &self.body {
            if let Some(head) = body.first() {
                return Some(Vector2D {
                    x: head.x + self.direction.unwrap().0,
                    y: head.y + self.direction.unwrap().1,
                });
            }
        }

        None
    }

    pub fn move_forward(&mut self) {
        let mut new_head = self.get_new_head().unwrap();

        if new_head.x < 0 {
            new_head.x = Map::SIZE.0 as i8 - 1;
        } else if new_head.x >= Map::SIZE.0 as i8 {
            new_head.x = 0;
        }

        if new_head.y < 0 {
            new_head.y = Map::SIZE.1 as i8 - 1;
        } else if new_head.y >= Map::SIZE.1 as i8 {
            new_head.y = 0;
        }

        if let Some(body) = &self.body {
            if body.contains(&new_head) {
                GAME_INSTANCE.lock().unwrap().restart(self);
                return;
            }
        }

        let mut game_instance = GAME_INSTANCE.lock().unwrap();

        if let Some(food_pos) = game_instance.get_food_pos() {
            if new_head == food_pos {
                game_instance.remove_food();
                game_instance.generate_food();
                self.grow();
            }
        }

        if let Some(body) = self.body.as_mut() {
            body.insert(0, new_head);
            body.pop();
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

    const SIZE: u8 = 2;
}
