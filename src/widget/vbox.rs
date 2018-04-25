use pane::Pane;
use widget::{Bound, BoundSize, Widget};
use {Position, Size};

// TODO need to hold on to Rc, so users can still access contents
pub struct VBox {
    pub contents: Vec<Box<Widget>>,
}

impl VBox {
    pub fn new() -> VBox {
        VBox { contents: vec![] }
    }
}

impl Widget for VBox {
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        let bounds: Vec<BoundSize> = self.contents.iter().map(|w| w.render_bounds()).collect();
        let (fixed_height, free_count) = coalesce_bounds(bounds.iter().map(|b| b.height).collect());
        let free_height = size.height - fixed_height;
        let step = (free_height as f32 / free_count as f32).ceil() as u16;
        let mut pool = free_height;
        let mut children = vec![];
        let mut counter = 1;
        let mut log = vec![];
        for item in self.contents.iter() {
            let height = match item.render_bounds().height {
                Bound::Fixed(n) => n,
                Bound::AtLeast(n) => {
                    log.push(format!("step={} pool={}", step, pool));
                    let extra = if step < pool {
                        pool -= step;
                        step
                    } else {
                        let tmp = pool;
                        pool = 0;
                        tmp
                    };
                    log.push(format!("extra={}", extra));
                    n + extra
                }
            };
            children.push(item.render(Position::new(1, counter), Size::new(size.width, height)));
            counter += height;
        }
        /*let dbg = Pane {
            position: Position::new(60,1),
            content: Some(log),
            focus: None,
            children: None,
        };
        children.push(dbg);*/
        Some(children)
    }
    fn render_bounds(&self) -> BoundSize {
        let bounds: Vec<BoundSize> = self.contents.iter().map(|w| w.render_bounds()).collect();
        let (width, _) = coalesce_bounds(bounds.iter().map(|b| b.width).collect());
        let (height, _) = coalesce_bounds(bounds.iter().map(|b| b.height).collect());
        BoundSize {
            width: Bound::AtLeast(width),
            height: Bound::AtLeast(height),
        }
    }
}

fn coalesce_bounds(bounds: Vec<Bound>) -> (u16, usize) {
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
