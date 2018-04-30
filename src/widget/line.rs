use widget::{Bound, BoundSize, Direction, Widget};
use Size;

use std::iter::repeat;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub direction: Direction,
}

impl Widget for Line {
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        let lines = match self.direction {
            Direction::Horizontal => vec![repeat("─").take(size.width as usize).collect()],
            Direction::Vertical => repeat("│".to_owned())
                .take(size.height as usize)
                .collect(),
        };
        Some(lines)
    }
    fn render_bounds(&self) -> BoundSize {
        match self.direction {
            Direction::Horizontal => BoundSize {
                width: Bound::AtLeast(1),
                height: Bound::Fixed(1),
            },
            Direction::Vertical => BoundSize {
                width: Bound::Fixed(1),
                height: Bound::AtLeast(1),
            },
        }
    }
}

impl Line {
    pub fn new(direction: Direction) -> Self {
        Line { direction }
    }
    pub fn horizontal() -> Self {
        Line {
            direction: Direction::Horizontal,
        }
    }
    pub fn vertical() -> Self {
        Line {
            direction: Direction::Vertical,
        }
    }
    pub fn flip(&mut self) {
        self.direction = match self.direction {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        };
    }
}
