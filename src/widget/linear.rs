use pane::Pane;
use widget::{Bound, BoundSize, Shared, Widget};
use {Position, Size};

#[derive(Debug, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}
// TODO need to hold on to Rc, so users can still access contents
pub struct Linear {
    pub contents: Vec<Box<Widget>>,
    pub direction: Direction,
}

impl Linear {
    pub fn new(contents: Vec<Box<Widget>>, direction: Direction) -> Linear {
        Linear {
            contents,
            direction,
        }
    }
    pub fn flip(&mut self) {
        match self.direction {
            Direction::Horizontal => self.direction = Direction::Vertical,
            Direction::Vertical => self.direction = Direction::Horizontal,
        }
    }
    pub fn vbox() -> Linear {
        Linear {
            contents: vec![],
            direction: Direction::Vertical,
        }
    }
    pub fn hbox() -> Linear {
        Linear {
            contents: vec![],
            direction: Direction::Horizontal,
        }
    }
    pub fn push(&mut self, w: &Shared<impl Widget + 'static>) {
        self.contents.push(Box::new(w.clone()));
    }
}

fn layout_bounds_proportional(bounds: &[Bound], goal: u16) -> Vec<u16> {
    let (fixed_size, free_count) = coalesce_bounds(bounds);
    let free_size = goal - fixed_size;
    let step = (f32::from(free_size) / free_count as f32).ceil() as u16;
    let mut pool = free_size;
    let mut sizes = vec![];
    for item in bounds {
        match item {
            Bound::Fixed(n) => sizes.push(*n),
            Bound::AtLeast(n) => {
                if step < pool {
                    sizes.push(*n + step);
                    pool -= step;
                } else {
                    sizes.push(*n + pool);
                    pool = 0;
                };
            }
        };
    }
    sizes
}

impl Widget for Linear {
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        let mut counter = 0;
        let children: Vec<Pane> = match &self.direction {
            Direction::Vertical => layout_bounds_proportional(
                &self.contents
                    .iter()
                    .map(|w| w.render_bounds().height)
                    .collect::<Vec<Bound>>(),
                size.height,
            ).into_iter()
                .zip(self.contents.iter())
                .map(|(itemsize, item)| {
                    let tmp = counter;
                    counter += itemsize;
                    item.render(Position::new(0, tmp), Size::new(size.width, itemsize))
                })
                .collect(),
            Direction::Horizontal => layout_bounds_proportional(
                &self.contents
                    .iter()
                    .map(|w| w.render_bounds().width)
                    .collect::<Vec<Bound>>(),
                size.width,
            ).into_iter()
                .zip(self.contents.iter())
                .map(|(itemsize, item)| {
                    let tmp = counter;
                    counter += itemsize;
                    item.render(Position::new(tmp, 0), Size::new(itemsize, size.height))
                })
                .collect(),
        };
        Some(children)
    }
    fn render_bounds(&self) -> BoundSize {
        let bounds: Vec<BoundSize> = self.contents.iter().map(|w| w.render_bounds()).collect();
        let (width, _) = coalesce_bounds(&bounds.iter().map(|b| b.width).collect::<Vec<Bound>>());
        let (height, _) = coalesce_bounds(&bounds.iter().map(|b| b.height).collect::<Vec<Bound>>());
        BoundSize {
            width: Bound::AtLeast(width),
            height: Bound::AtLeast(height),
        }
    }
}

fn coalesce_bounds(bounds: &[Bound]) -> (u16, usize) {
    let mut fixed: u16 = 0;
    let mut free: usize = 0;
    for bound in bounds {
        match bound {
            Bound::Fixed(n) => fixed += n,
            Bound::AtLeast(n) => {
                fixed += n;
                free += 1
            }
        }
    }
    (fixed, free)
}
