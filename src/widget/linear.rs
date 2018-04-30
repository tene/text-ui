use pane::Pane;
use widget::{Bound, BoundSize, Direction, Shared, Widget};
use {Position, Size};

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
    let free_size = (goal) - fixed_size;
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
        let get_bound = |b: BoundSize| match &self.direction {
            Direction::Vertical => b.height,
            Direction::Horizontal => b.width,
        };
        let total_size = match &self.direction {
            Direction::Vertical => size.height,
            Direction::Horizontal => size.width,
        };
        let build_pos = |offset| match &self.direction {
            Direction::Vertical => Position::new(0, offset),
            Direction::Horizontal => Position::new(offset, 0),
        };
        let build_size = |itemsize| match &self.direction {
            Direction::Vertical => Size::new(size.width, itemsize),
            Direction::Horizontal => Size::new(itemsize, size.height),
        };
        let children: Vec<Pane> = layout_bounds_proportional(
            &self.contents
                .iter()
                .map(|w| get_bound(w.render_bounds()))
                .collect::<Vec<Bound>>(),
            total_size,
        ).into_iter()
            .zip(self.contents.iter())
            .map(|(itemsize, item)| {
                let tmp = counter;
                counter += itemsize;
                item.render(build_pos(tmp), build_size(itemsize))
            })
            .collect();
        Some(children)
    }
    fn render_bounds(&self) -> BoundSize {
        let (width_iter, height_iter): (Vec<Bound>, Vec<Bound>) = self.contents
            .iter()
            .map(|w| w.render_bounds().split())
            .unzip();
        let (width, _) = coalesce_bounds(&width_iter);
        let (height, _) = coalesce_bounds(&height_iter);
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
