use crate::snake::Snake;
use crate::storage::Storage;
use crate::utils::Vector2D;
use crossterm::style::{Color, Stylize};
use rand::Rng;
use std::io::{stdout, Write};
use std::time::Duration;

pub struct Game {
    snake: Snake,
    food_pos: Option<Vector2D<i16>>,
    food_index: u8,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(),
            food_pos: None,
            food_index: 0,
        };

        game.generate_food();
        game
    }

    pub fn get_snake(&mut self) -> &mut Snake {
        &mut self.snake
    }

    pub fn get_food_pos(&self) -> Option<Vector2D<i16>> {
        self.food_pos
    }

    pub fn remove_food(&mut self) {
        self.food_pos = None
    }

    pub fn generate_food(&mut self) -> Option<Vector2D<i16>> {
        self.food_index = rand::rng().random_range(0..=Self::FOODS.len() - 1) as u8;

        let x = rand::rng().random_range(0..Self::MAP_SIZE.0);
        let y = rand::rng().random_range(0..Self::MAP_SIZE.1);

        self.food_pos = Some(Vector2D {
            x: x as i16,
            y: y as i16,
        });

        self.food_pos
    }

    pub fn restart(&mut self) {
        let snake = &mut self.snake;

        if let Some(body) = &mut snake.body {
            body.clear();
        }
        snake.set_body_pos(snake.get_center_body_pos());
        snake.set_direction(snake.get_random_direction());
        snake.score = 0;

        self.remove_food();
        self.generate_food();
    }

    pub fn get_food_char(&self) -> char {
        Self::FOODS[self.food_index as usize]
    }

    pub fn pickup_food(&mut self) {
        let snake = &mut self.snake;
        snake.grow();

        snake.score += 1;

        if snake.score > snake.highest_score {
            snake.highest_score = snake.score;
            Storage::save_highest_score(snake).unwrap();
        }

        self.remove_food();
        self.generate_food();
    }

    pub fn update(&mut self) {
        let food_pos = self.get_food_pos();
        let new_head = self.snake.get_new_head();

        self.snake.move_forward();

        if let Some(new_head) = new_head {
            let collides_with_body = if let Some(body) = &self.snake.body {
                body.contains(&new_head)
            } else {
                false
            };

            if collides_with_body {
                self.restart();
                return;
            }

            if let Some(food_pos) = food_pos {
                if new_head == food_pos {
                    self.pickup_food();
                }
            }

            if let Some(body) = &mut self.snake.body {
                body.insert(0, new_head);
                body.pop();
            }
        }
    }

    pub fn draw(&mut self) {
        let mut stdout = stdout();
        let food_pos = self.get_food_pos();
        let food_char = self.get_food_char();

        let snake = &mut self.snake;
        let body = snake.body.as_ref().map(|b| b.to_vec());
        let body_colors = snake.get_body_colors();

        if let Some(body) = body {
            if let Some(body_colors) = body_colors {
                let mut score = format!("Score: {}", snake.score);

                if snake.highest_score > 0 {
                    score.push_str(&format!(" - Highest Score: {}", snake.highest_score));
                }

                println!("{}", score);

                let horizontal_line = "───".repeat(Self::MAP_SIZE.0 as usize + 1);
                println!("┌{}┐", horizontal_line);

                for y in 0..=Self::MAP_SIZE.1 {
                    print!("│");
                    for x in 0..=Self::MAP_SIZE.0 {
                        let pos = Vector2D {
                            x: x as i16,
                            y: y as i16,
                        };

                        if let Some(body_index) = body.iter().position(|&p| p == pos) {
                            let body_color = body_colors
                                .get(body_index)
                                .unwrap_or(&Game::SNAKE_HEAD_COLOR);

                            let (r, g, b) = match body_color {
                                Color::Rgb { r, g, b } => (*r, *g, *b),
                                _ => panic!("Expected RGB color"),
                            };

                            print!(" {} ", Game::SNAKE.with(Color::Rgb { r, g, b }));
                        } else if food_pos == Some(pos) {
                            print!("{} ", food_char);
                        } else {
                            print!("   ");
                        }
                    }
                    println!("│");
                }

                println!("└{}┘", horizontal_line);
            }
        }

        stdout.flush().unwrap();
    }

    pub const UPDATE_INTERVAL: Duration = Duration::from_millis(200);
    pub const FOODS: [char; 2] = ['🍎', '🍏'];
    pub const SNAKE: char = '■';
    pub const SNAKE_HEAD_COLOR: Color = Color::Rgb { r: 0, g: 255, b: 0 };
    pub const SNAKE_TAIL_COLOR: Color = Color::Rgb { r: 255, g: 0, b: 0 };
    pub const MAP_SIZE: (u16, u16) = (20, 20);
}
