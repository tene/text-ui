use std::fmt;
use std::iter::repeat;

use {Direction, GrowthPolicy, Name, RenderBackend, RenderElement, Widget, WidgetRenderContext};

pub struct Linear<'a, B, N>
where
    B: 'a + RenderBackend<N>,
    N: 'a + Name,
{
    widgets: Vec<&'a dyn Widget<B, N>>,
    direction: Direction,
}

impl<'a, B, N> Linear<'a, B, N>
where
    B: 'a + RenderBackend<N>,
    N: 'a + Name,
{
    pub fn new(direction: Direction, widgets: Vec<&'a dyn Widget<B, N>>) -> Self {
        Self { direction, widgets }
    }
    pub fn hbox(widgets: Vec<&'a dyn Widget<B, N>>) -> Self {
        Self::new(Direction::Horizontal, widgets)
    }
    pub fn vbox(widgets: Vec<&'a dyn Widget<B, N>>) -> Self {
        Self::new(Direction::Vertical, widgets)
    }
}

impl<'a, B, N> Widget<B, N> for Linear<'a, B, N>
where
    B: 'a + RenderBackend<N>,
    N: 'a + Name,
{
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        let dir = self.direction;
        // Clippy complains about a complex type here; I'm not convinced it's an improvement.
        type Segments<T> = (Vec<(usize, T)>, Vec<(usize, T)>);
        let (fixed, greedy): Segments<&dyn Widget<B, N>> = self
            .widgets
            .clone()
            .into_iter()
            .enumerate()
            .partition(|(_, w)| {
                w.growth_policy().in_direction(self.direction) == GrowthPolicy::FixedSize
            });
        let mut remaining_size = ctx
            .bound()
            .in_direction(dir)
            .expect("Linear layout without on-axis constraint");
        let fixed_constraint = ctx
            .bound()
            .against_direction(dir)
            .expect("Linear layout without off-axis constraint");
        let greedy_count = greedy.len();
        let free_bound = ctx.bound().free_direction(dir);
        let mut blocks: Vec<(usize, B::Element)> = fixed
            .into_iter()
            .map(|(i, w)| {
                // XXX TODO Maybe should be fetching bounds from children?
                let b = ctx.render_sized(free_bound, w);
                assert_eq!(b.size().against_direction(dir), fixed_constraint);
                remaining_size -= b.size().in_direction(dir);
                (i, b)
            }).collect();
        // XXX TODO UPDATE THIS when we get bounds from children
        let chunk_size = remaining_size / greedy_count;
        let sizes = repeat(chunk_size + 1)
            .take(remaining_size % greedy_count)
            .chain(repeat(chunk_size));
        blocks.extend(greedy.into_iter().zip(sizes).map(|((i, w), size)| {
            let bound = free_bound.constrain_direction(dir, size);
            let b = ctx.render_sized(bound, w);
            (i, b)
        }));
        blocks.sort_by_key(|a| a.0);
        let mut blocks = blocks.into_iter().map(|(_, b)| b);
        let init = blocks.next().expect("Linear layout with no children");
        blocks.fold(init, |acc, b| acc.concat_dir(dir, b))
    }
}

impl<'a, B, N> fmt::Debug for Linear<'a, B, N>
where
    B: 'a + RenderBackend<N>,
    N: 'a + Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Linear {{ direction: {:?}, widgets: {:?}}}",
            self.direction, self.widgets
        )
    }
}
