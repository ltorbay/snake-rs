use std::collections::LinkedList;
use crate::direction::Direction;
use crate::position::Position;

pub struct Snake {
    pub body: LinkedList<Position>,
    pub dir: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back(Position { x: 10, y: 10 });
        Snake {
            body,
            dir: Direction::Right,
        }
    }

    pub fn update(&mut self) {
        let head = self.body.front().unwrap().clone();
        
        // Calculate potential next position
        let (mut next_x, mut next_y) = (head.x, head.y);
        match self.dir {
            Direction::Up => next_y -= 1,
            Direction::Down => next_y += 1,
            Direction::Left => next_x -= 1,
            Direction::Right => next_x += 1,
        }
        
        // Wrap around edges
        let grid_width = crate::WINDOW_WIDTH / crate::SQUARE_WIDTH;
        let grid_height = crate::WINDOW_HEIGHT / crate::SQUARE_WIDTH;
        
        if next_x < 0 {
            next_x = grid_width - 1;
        } else if next_x >= grid_width {
            next_x = 0;
        }

        if next_y < 0 {
            next_y = grid_height - 1;
        } else if next_y >= grid_height {
            next_y = 0;
        }
        
        self.body.push_front(Position { x: next_x, y: next_y });
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        let tail = self.body.back().unwrap().clone();
        self.body.push_back(tail);
    }

    pub fn head_position(&self) -> Position {
        self.body.front().unwrap().clone()
    }

    pub fn collides_with_self(&self) -> bool {
        let head = self.head_position();
        for (i, segment) in self.body.iter().enumerate() {
            if i != 0 && segment.x == head.x && segment.y == head.y {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_snake() {
        let snake = Snake::new();
        assert_eq!(snake.body.len(), 1);
        assert_eq!(snake.head_position(), Position { x: 10, y: 10 });
        assert!(matches!(snake.dir, Direction::Right));
    }

    #[test]
    fn test_snake_movement() {
        let mut snake = Snake::new();
        
        // Test right movement (default)
        snake.update();
        assert_eq!(snake.head_position(), Position { x: 11, y: 10 });

        // Test downward movement
        snake.dir = Direction::Down;
        snake.update();
        assert_eq!(snake.head_position(), Position { x: 11, y: 11 });

        // Test left movement
        snake.dir = Direction::Left;
        snake.update();
        assert_eq!(snake.head_position(), Position { x: 10, y: 11 });

        // Test upward movement
        snake.dir = Direction::Up;
        snake.update();
        assert_eq!(snake.head_position(), Position { x: 10, y: 10 });
    }

    #[test]
    fn test_snake_growth() {
        let mut snake = Snake::new();
        let initial_length = snake.body.len();
        
        snake.grow();
        assert_eq!(snake.body.len(), initial_length + 1);
        
        // Verify position after growing and moving
        snake.update();
        assert_eq!(snake.body.len(), initial_length + 1);
        assert_eq!(snake.head_position(), Position { x: 11, y: 10 });
    }

    #[test]
    fn test_snake_collision() {
        let mut snake = Snake::new();
        
        // Grow the snake a few times to make it long enough for self collision
        for _ in 0..5 {
            snake.grow();
        }
        
        // Create a path that leads to self collision
        snake.update(); // Right
        snake.update(); // Right again
        snake.dir = Direction::Down;
        snake.update();
        snake.dir = Direction::Left;
        snake.update();
        snake.dir = Direction::Up;
        snake.update();
        
        assert!(snake.collides_with_self());
    }

    #[test]
    fn test_snake_wrapping() {
        let mut snake = Snake::new();
        
        // Test wrapping right to left
        for _ in 0..30 {
            snake.update();
        }
        assert_eq!(snake.head_position().x, 0);
        
        // Test wrapping left to right
        snake.dir = Direction::Left;
        snake.update();
        assert_eq!(snake.head_position().x, (crate::WINDOW_WIDTH / crate::SQUARE_WIDTH) - 1);
        
        // Test wrapping bottom to top
        snake = Snake::new();
        snake.dir = Direction::Down;
        for _ in 0..30 {
            snake.update();
        }
        assert_eq!(snake.head_position().y, 0);
        
        // Test wrapping top to bottom
        snake.dir = Direction::Up;
        snake.update();
        assert_eq!(snake.head_position().y, (crate::WINDOW_HEIGHT / crate::SQUARE_WIDTH) - 1);
    }
}
