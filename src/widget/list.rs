use std::fmt;
use std::fmt::Debug;

use itertools::{unfold, Itertools};

use {Direction, Name, RenderContext, TextBlock, Widget};

pub trait WidgetEnumerable<N: Name>: Debug {
    fn get_widget(&self, index: usize) -> Option<&dyn Widget<N>>;
}

impl<N: Name, W: Widget<N>> WidgetEnumerable<N> for &[W] {
    fn get_widget(&self, index: usize) -> Option<&dyn Widget<N>> {
        self.get(index).map(|w| w as &dyn Widget<N>)
    }
}

impl<N: Name, W: Widget<N>> WidgetEnumerable<N> for Vec<W> {
    fn get_widget(&self, index: usize) -> Option<&dyn Widget<N>> {
        self.get(index).map(|w| w as &dyn Widget<N>)
    }
}

pub struct List<N: Name, E: WidgetEnumerable<N>> {
    name: Option<N>,
    container: E,
    index: usize, // XXX TODO Shared(usize)
    offset: i32,  // XXX TODO Shared(i32)
}

impl<N: Name, E: WidgetEnumerable<N>> List<N, E> {
    pub fn new(name: Option<N>, container: E, index: usize, offset: i32) -> Self {
        Self {
            name,
            container,
            index,
            offset,
        }
    }
}

impl<N: Name, E: WidgetEnumerable<N>> Widget<N> for List<N, E> {
    fn name(&self) -> Option<N> {
        self.name
    }
    fn widget_type(&self) -> &'static str {
        "List"
    }
    fn render(&self, ctx: RenderContext<N>) -> TextBlock<N> {
        let total_height = ctx
            .bound()
            .height
            .expect("List without vertical constraint") as i32;
        let width_bound = ctx.bound().free_direction(Direction::Vertical);
        let body = ctx.render_sized(
            width_bound,
            self.container.get_widget(self.index).expect(&format!(
                "No widget at index: ({:?}, {:?})",
                self.name, self.index
            )),
        );
        let mut tail_length = total_height - (body.size.rows as i32 + self.offset);
        let mut head_length = self.offset;
        let tail: Option<TextBlock<N>> = unfold(self.index + 1, |i| {
            let tb = self
                .container
                .get_widget(*i)
                .map(|w| ctx.render_sized(width_bound, w));
            *i += 1;
            tb
        }).take_while(|tb| {
            if tail_length <= 0 {
                false
            } else {
                tail_length -= tb.size.rows as i32;
                true
            }
        }).fold1(|acc, next| acc.vconcat(next));
        let head: Option<TextBlock<N>> = unfold(self.index, |i| {
            let prev_idx = self.index.checked_sub(1)?;
            *i = prev_idx;
            self.container
                .get_widget(prev_idx)
                .map(|w| ctx.render_sized(width_bound, w))
        }).take_while(|tb| {
            if head_length <= 0 {
                false
            } else {
                head_length -= tb.size.rows as i32;
                true
            }
        }).fold1(|acc, prev| prev.vconcat(acc));
        let body = match head {
            Some(mut head) => {
                let extra_rows = head.size.rows - self.offset as usize;
                head.trim_top(extra_rows).vconcat(body)
            }
            None => {
                if self.offset < 0 {
                    body.trim_top(self.offset.abs() as usize)
                } else {
                    body
                }
            }
        };
        match tail {
            Some(tail) => {
                let tb = body.vconcat(tail);
                if tb.size.rows > total_height as usize {
                    let extra_rows = tb.size.rows - total_height as usize;
                    tb.trim_bottom(extra_rows)
                } else {
                    tb
                }
            }
            None => body,
        }
    }
    // XXX TODO add scroll, click, and keyboard handlers
}

impl<N: Name, E: WidgetEnumerable<N>> fmt::Debug for List<N, E>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "List {{ name: {:?}, index: {:?}, offset: {:?}, container: {:?} }}",
            self.name, self.index, self.offset, self.container
        )
    }
}
