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
            MoveDirection::LEFT => self.pos.x = (self.pos.x - 1).max(0),
            MoveDirection::RIGHT => self.pos.x = (self.pos.x + 1).min(Map::WIDTH),
            MoveDirection::FORWARD => self.pos.y = (self.pos.y - 1).max(0),
            MoveDirection::BACKWARD => self.pos.y = (self.pos.y + 1).min(Map::HEIGHT),
        }
    }

    pub const CHAR: char = '■';
}
