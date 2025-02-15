use crate::game::Game;
use crate::storage::Storage;
use crate::utils;
use crate::utils::Utils;
use crossterm::style::Color;
use rand::Rng;
use utils::Vector2D;

pub struct Snake {
    pub body: Option<Vec<Vector2D<i16>>>,
    pub direction: Option<(i8, i8)>,
    pub score: u16,
    pub highest_score: u16,
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake = Snake {
            body: None,
            direction: None,
            score: 0,
            highest_score: 0,
        };

        snake.body = snake.get_center_body_pos();
        snake.direction = snake.get_random_direction();

        match Storage::load_highest_score() {
            Ok(highest_score) => {
                if highest_score == 0 {
                    let _ = Storage::save_highest_score(&snake);
                } else {
                    snake.highest_score = highest_score;
                }
            }
            Err(_) => {}
        }

        snake
    }

    pub fn get_random_direction(&self) -> Option<(i8, i8)> {
        let random_direction_index = rand::rng().random_range(0..Self::RANDOM_DIRECTIONS.len());
        Some(Self::RANDOM_DIRECTIONS[random_direction_index])
    }

    pub fn set_direction(&mut self, direction: Option<(i8, i8)>) {
        self.direction = direction;
    }

    pub fn get_center_body_pos(&self) -> Option<Vec<Vector2D<i16>>> {
        let mut body: Vec<Vector2D<i16>> = Vec::new();

        for i in 0..Self::SIZE {
            body.push(Vector2D {
                x: (i + (Game::MAP_SIZE.0 / 2)) as i16,
                y: (Game::MAP_SIZE.1 / 2) as i16,
            });
        }

        Some(body)
    }

    pub fn set_body_pos(&mut self, pos: Option<Vec<Vector2D<i16>>>) {
        self.body = pos;
    }

    pub fn get_new_head(&self) -> Option<Vector2D<i16>> {
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
                    index as f32 / (body.len() - 1) as f32,
                    Game::SNAKE_HEAD_COLOR,
                    Game::SNAKE_TAIL_COLOR,
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
