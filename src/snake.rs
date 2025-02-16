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

