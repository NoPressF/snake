use crate::map::Map;
use crate::utils;
use utils::Vector2D;

pub struct Player {
    pub body: Vec<Vector2D>,
    pub direction: (i8, i8),
}

impl Player {
    pub(crate) fn new() -> Player {
        let size: u8 = 4;
        let mut body = Vec::new();

        for i in 0..size {
            body.push(Vector2D {
                x: i as i8 + (Map::SIZE.0 / 2) as i8,
                y: (Map::SIZE.1 / 2) as i8,
            });
        }

        Player {
            body,
            direction: (1, 0),
        }
    }

    fn get_new_head(&self) -> Vector2D {
        let head = *self.body.first().unwrap();
        Vector2D {
            x: head.x + self.direction.0,
            y: head.y + self.direction.1,
        }
    }

    pub fn move_forward(&mut self) {
        let mut new_head = self.get_new_head();

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

        self.body.insert(0, new_head);
        self.body.pop();
    }
    // pub fn grow(&mut self) {
    //     let new_head = self.get_new_head();
    //     self.body.insert(0, new_head);
    // }

    pub fn change_direction(&mut self, new_direction: (i8, i8)) {
        if self.direction.0 + new_direction.0 == 0 && self.direction.1 + new_direction.1 == 0 {
            return;
        }

        self.direction = new_direction;
    }

    pub const CHAR: char = '■';
}
