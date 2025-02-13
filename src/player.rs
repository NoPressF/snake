use crate::map::Map;
use crate::utils;
use utils::Vector2D;

pub enum MoveDirection {
    FORWARD,
    LEFT,
    BACKWARD,
    RIGHT,
}

pub struct Player {
    pub pos: Vector2D,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player {
            pos: Vector2D { x: 0, y: 0 },
        }
    }

    pub fn move_to(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::LEFT => {
                self.pos.x = (self.pos.x.saturating_sub(1)).clamp(-1, Map::WIDTH as i8);

                if self.pos.x == -1 {
                    self.pos.x = Map::WIDTH as i8;
                }
            },
            MoveDirection::FORWARD => {
                self.pos.y = (self.pos.y.saturating_sub(1)).clamp(-1, Map::HEIGHT as i8);

                if self.pos.y == -1 {
                    self.pos.y = Map::HEIGHT as i8;
                }
            }
            MoveDirection::RIGHT => {
                self.pos.x = (self.pos.x + 1).clamp(-1, Map::WIDTH as i8 + 1);

                if self.pos.x == Map::WIDTH as i8 + 1 {
                    self.pos.x = 0;
                }
            },
            MoveDirection::BACKWARD => {
                self.pos.y = (self.pos.y + 1).clamp(-1, Map::HEIGHT as i8 + 1);

                if self.pos.y == Map::HEIGHT as i8 + 1 {
                    self.pos.y = 0;
                }
            },
        }
    }

    pub const CHAR: char = '■';
}
