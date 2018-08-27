use std::iter::repeat;
use {
    Direction, FullGrowthPolicy, Name, RenderBackend, Segment, TextBlock, Widget,
    WidgetRenderContext,
};

#[derive(Debug)]
pub struct Line {
    direction: Direction,
}

impl Line {
    pub fn vertical() -> Self {
        Self {
            direction: Direction::Vertical,
        }
    }
    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
        }
    }
}

impl<B, N> Widget<B, N> for Line
where
    B: RenderBackend<N>,
    N: Name,
{
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        let count = ctx
            .bound()
            .in_direction(self.direction)
            .expect("Line rendered without appropriate constraint");
        let ctx = ctx.with_bound(ctx.bound().constrain_against(self.direction, 1));
        match self.direction {
            Direction::Horizontal => {
                let line = Segment::new(
                    None,
                    "Line",
                    "Horizontal",
                    repeat('─').take(count).collect::<String>(),
                );
                ctx.clip_line(line)
            }
            Direction::Vertical => {
                let rows: Vec<String> = repeat("│".to_owned()).take(count).collect();
                let text = TextBlock::new_lines(None, "Line", "Vertical", rows);
                ctx.text(text)
            }
        }
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        match self.direction {
            Direction::Horizontal => FullGrowthPolicy::fixed_height(),
            Direction::Vertical => FullGrowthPolicy::fixed_width(),
        }
    }
    fn name(&self) -> Option<N> {
        None
    }
}
