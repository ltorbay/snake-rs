extern crate piston_window;
extern crate rand;

mod position;
mod direction;
mod snake;

use piston_window::*;
use rand::Rng;

use position::Position;
use direction::Direction;
use snake::Snake;

pub const SQUARE_WIDTH: i32 = 20;
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;

struct Food {
    pos: Position,
}

struct Game {
    snake: Snake,
    food: Food,
    game_over: bool,
}


impl Food {
    fn new(snake: &Snake) -> Food {
        let mut rng = rand::thread_rng();
        let max_pos = (WINDOW_WIDTH / SQUARE_WIDTH) - 1;

        loop {
            let pos = Position {
                x: rng.gen_range(0..=max_pos),
                y: rng.gen_range(0..=max_pos),
            };
            
            if !snake.body.iter().any(|p| p.x == pos.x && p.y == pos.y) {
                return Food { pos };
            }
        }
    }
}

impl Game {
    fn new() -> Game {
        let snake = Snake::new();
        let food = Food::new(&snake);
        Game {
            snake,
            food,
            game_over: false,
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.snake.update();

        if self.snake.collides_with_self() {
            self.game_over = true;
            return;
        }

        let head = self.snake.head_position();
        if head.x == self.food.pos.x && head.y == self.food.pos.y {
            self.snake.grow();
            self.food = Food::new(&self.snake);
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();
    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();
    let mut events = Events::new(EventSettings::new().ups(8));

    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if !game.game_over {
                match key {
                    Key::Up if game.snake.dir != Direction::Down => 
                        game.snake.dir = Direction::Up,
                    Key::Down if game.snake.dir != Direction::Up => 
                        game.snake.dir = Direction::Down,
                    Key::Left if game.snake.dir != Direction::Right => 
                        game.snake.dir = Direction::Left,
                    Key::Right if game.snake.dir != Direction::Left => 
                        game.snake.dir = Direction::Right,
                    _ => {}
                }
            }
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);

            // Draw snake
            let segments: Vec<_> = game.snake.body.iter().collect();
            for (i, pos) in segments.iter().enumerate() {
                let offset = 2; // Space between segments
                let size = if i == 0 { SQUARE_WIDTH } else { SQUARE_WIDTH - 4 };
                let color = if i == 0 {
                    [0.0, 0.8, 0.0, 1.0] // Brighter green for head
                } else {
                    [0.0, 0.6, 0.0, 1.0] // Darker green for body
                };

                // Add offset to create spacing between segments
                rectangle(
                    color,
                    [
                        (pos.x * SQUARE_WIDTH + offset) as f64,
                        (pos.y * SQUARE_WIDTH + offset) as f64,
                        (size - offset * 2) as f64,
                        (size - offset * 2) as f64
                    ],
                    context.transform,
                    graphics,
                );
            }

            // Draw food with glow effect
            let food_size = SQUARE_WIDTH - 4;
            let food_offset = 2;

            // Draw glow effect
            rectangle(
                [1.0, 0.0, 0.0, 0.3],
                [
                    (game.food.pos.x * SQUARE_WIDTH) as f64,
                    (game.food.pos.y * SQUARE_WIDTH) as f64,
                    SQUARE_WIDTH as f64,
                    SQUARE_WIDTH as f64
                ],
                context.transform,
                graphics,
            );

            // Draw food
            rectangle(
                [1.0, 0.0, 0.0, 0.8],
                [
                    (game.food.pos.x * SQUARE_WIDTH + food_offset) as f64,
                    (game.food.pos.y * SQUARE_WIDTH + food_offset) as f64,
                    (food_size) as f64,
                    (food_size) as f64
                ],
                context.transform,
                graphics,
            );

            if game.game_over {
                Text::new_color([1.0, 0.0, 0.0, 1.0], 32)
                    .draw(
                        "Game Over!",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(300.0, 300.0),
                        graphics
                    )
                    .unwrap();
            }
            
            glyphs.factory.encoder.flush(device);
        });
    }
}
