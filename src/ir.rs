use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;

use unicode_segmentation::UnicodeSegmentation;

use indextree::IndexTree;
use {
    App, Color, Direction, EventContext, Key, KeyCallback, MouseCallback, MouseEvent, Name, Pos,
    RenderBound, Size,
};

// XXX TODO Better name??
#[derive(Debug, Clone, Copy)]
pub struct ContentID<N: Name> {
    pub name: Option<N>,
    pub widget_type: &'static str,
    pub class: &'static str,
}

impl<N: Name> ContentID<N> {
    pub fn as_tuple(self) -> (Option<N>, &'static str, &'static str) {
        (self.name, self.widget_type, self.class)
    }
    pub fn new(name: Option<N>, widget_type: &'static str, class: &'static str) -> Self {
        Self {
            name,
            widget_type,
            class,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment<N: Name> {
    pub id: ContentID<N>,
    pub text: String,
    pub len: usize,
}

impl<N: Name> Segment<N> {
    pub fn new(name: Option<N>, widget: &'static str, class: &'static str, text: String) -> Self {
        let id = ContentID::new(name, widget, class);
        Self::new_id(id, text)
    }
    pub fn new_id(id: ContentID<N>, text: String) -> Self {
        let len = UnicodeSegmentation::graphemes(text.as_str(), true).count();
        Self { id, text, len }
    }
    pub fn blank_sized(id: ContentID<N>, len: usize) -> Self {
        let text: String = repeat(' ').take(len).collect();
        Self { id, text, len }
    }
}

#[derive(Debug, Clone)]
pub struct TextLine<N: Name> {
    pub segments: Vec<Segment<N>>,
    pub len: usize,
}

impl<N: Name> TextLine<N> {
    pub fn clip(mut self, len: usize) -> Self {
        if self.len < len {
            let last_id = (&self.segments)
                .last()
                .expect("Attempt to clip empty TextLine")
                .id;
            self.push(Segment::blank_sized(last_id, len - self.len));
        } else if self.len > len {
            let mut accum: usize = 0;
            self.segments = self
                .segments
                .into_iter()
                .filter_map(move |s| {
                    if accum >= len {
                        None
                    } else if accum + s.len <= len {
                        accum += s.len;
                        Some(s)
                    } else {
                        s.text = UnicodeSegmentation::graphemes(s.text.as_str(), true)
                            .take(len - accum)
                            .collect();
                        accum = len;
                        Some(s)
                    }
                }).collect();
            self.len = len;
        }
        self
    }
    pub fn push(&mut self, segment: Segment<N>) {
        self.len += segment.len;
        self.segments.push(segment);
    }
    pub fn hconcat(&mut self, mut other: Self) {
        self.len += other.len;
        self.segments.append(&mut other.segments);
    }
}

impl<N: Name> From<Segment<N>> for TextLine<N> {
    fn from(segment: Segment<N>) -> Self {
        let len = segment.len;
        let segments = vec![segment];
        Self { segments, len }
    }
}

pub struct TextBlock<N: Name> {
    pub lines: Vec<TextLine<N>>,
    pub size: Size,
    key_callbacks: IndexTree<N, KeyCallback<N>>,
    cursors: HashMap<N, Pos>,
    hit_map: Vec<Vec<Option<usize>>>,
    mouse_callbacks: IndexTree<usize, (Option<N>, Pos, MouseCallback<N>)>,
}

impl<N: Name> TextBlock<N> {
    pub(crate) fn new(lines: Vec<TextLine<N>>, width: usize, height: usize) -> Self {
        assert_eq!(lines.len(), height);
        let mut empty_line = Vec::new();
        // https://github.com/rust-lang/rust/issues/41758
        empty_line.resize(width, None);
        let mut hit_map = Vec::new();
        hit_map.resize(height, empty_line);
        let size = Size::new(width, height);
        Self {
            key_callbacks: IndexTree::new(),
            cursors: HashMap::new(),
            hit_map,
            mouse_callbacks: IndexTree::new(),
            lines,
            size,
        }
    }
    pub fn new_clipped(lines: Vec<TextLine<N>>, width: usize, height: usize) -> Self {
        let tb = Self::new(lines);
    }
    pub fn handle_key(&self, event_ctx: &EventContext<N>, focus: &N, key: Key) {
        use ShouldPropagate::*;
        for cb in self.key_callbacks.get_iter(focus) {
            match cb(event_ctx, key) {
                Stop => break,
                Continue => continue,
            }
        }
    }
    // XXX TODO Need to use internal mouse event type instead of termion's, with relative coords
    pub fn handle_mouse(&self, event_ctx: &EventContext<N>, mevent: MouseEvent) {
        use ShouldPropagate::*;
        let (x, y) = match mevent {
            MouseEvent::Press(_, x, y) => (x as usize - 1, y as usize - 1),
            MouseEvent::Release(x, y) => (x as usize - 1, y as usize - 1),
            MouseEvent::Hold(x, y) => (x as usize - 1, y as usize - 1),
        };
        if let Some(idx) = self.hit_map[y][x] {
            let frame_pos = Pos::new(x, y);
            for (_name, pos, cb) in self.mouse_callbacks.get_iter_idx(idx) {
                match cb(event_ctx, frame_pos - *pos, mevent) {
                    Stop => break,
                    Continue => continue,
                }
            }
        }
    }
    pub fn clip_lines(
        name: Option<N>,
        widget: &'static str,
        class: &'static str,
        lines: Vec<String>,
        bound: RenderBound,
    ) -> Self {
        let width = match bound.width {
            Some(width) => width,
            None => lines.iter().map(|l| l.len()).max().unwrap_or(0),
        };
        let id = ContentID::new(name, widget, class);
        let tl_iter = lines.into_iter().map(|l| {
            let tl: TextLine<N> = Segment::new_id(id, l).into();
            tl.clip(width)
        });
        let lines: Vec<TextLine<N>> = match bound.height {
            Some(height) => tl_iter
                .chain(repeat(Segment::blank_sized(id, width).into()))
                .take(height)
                .collect(),
            None => tl_iter.collect(),
        };
        Self::new(lines, width, lines.len())
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn add_key_handler(mut self, name: Option<N>, callback: KeyCallback<N>) -> Self {
        self.key_callbacks.push(name, callback);
        self
    }
    pub fn add_mouse_handler(mut self, name: Option<N>, callback: MouseCallback<N>) -> Self {
        let idx = self
            .mouse_callbacks
            .push(None, (name, Pos::new(0, 0), callback));
        for row in &mut self.hit_map {
            for cell in row {
                if cell.is_none() {
                    *cell = Some(idx);
                }
            }
        }
        self
    }
    pub fn add_cursor(mut self, name: N, pos: Pos) -> Self {
        self.cursors.insert(name, pos);
        self
    }
    pub fn get_cursor(&self, name: N) -> Option<Pos> {
        self.cursors.get(&name).cloned()
    }
    // Maybe factor out the common parts of these?
    pub fn vconcat(mut self, mut other: Self) -> Self {
        assert_eq!(self.size.cols, other.size.cols); // XXX TODO Maybe expand the smaller to fit?
        let pos_offset = Pos::new(0, self.size.rows);
        self.lines.append(&mut other.lines);

        self.key_callbacks.append(&mut other.key_callbacks);

        let mut offset_mouse_callbacks = other
            .mouse_callbacks
            .map(|(name, pos, cb)| (name, pos + pos_offset, cb));
        let idx_offset = self.mouse_callbacks.append(&mut offset_mouse_callbacks);
        for row in &mut other.hit_map {
            for cell in row {
                *cell = cell.map(|i| i + idx_offset);
            }
        }
        self.hit_map.append(&mut other.hit_map);

        self.cursors.extend(
            other
                .cursors
                .into_iter()
                .map(move |(n, p)| (n, p + pos_offset)),
        );
        self.size.rows += other.size.rows;
        self
    }
    pub fn hconcat(mut self, mut other: Self) -> Self {
        assert_eq!(self.size.rows, other.size.rows); // XXX TODO Maybe expand the smaller to fit?
        let pos_offset = Pos::new(self.size.rows, 0);
        for (mut a, b) in self.lines.iter_mut().zip(other.lines.into_iter()) {
            a.hconcat(b);
        }
        self.key_callbacks.append(&mut other.key_callbacks);

        let mut offset_mouse_callbacks = other
            .mouse_callbacks
            .map(|(name, pos, cb)| (name, pos + pos_offset, cb));
        let idx_offset = self.mouse_callbacks.append(&mut offset_mouse_callbacks);
        for row in &mut other.hit_map {
            for cell in row {
                *cell = cell.map(|i| i + idx_offset);
            }
        }

        for (mut a, b) in self.hit_map.iter_mut().zip(other.hit_map.into_iter()) {
            a.extend_from_slice(&b)
        }

        self.cursors.extend(
            other
                .cursors
                .into_iter()
                .map(move |(n, p)| (n, p + pos_offset)),
        );
        self.size.cols += other.size.cols;
        self
    }
    pub fn concat_dir(self, direction: Direction, other: Self) -> Self {
        match direction {
            Direction::Horizontal => self.hconcat(other),
            Direction::Vertical => self.vconcat(other),
        }
    }
    pub fn render_frame(&self, app: &App<N>, focus_name: Option<N>) -> Frame {
        let size = self.size;
        let focus = focus_name.and_then(|name| self.get_cursor(name));
        let image: Vec<FrameLine> = self
            .lines
            .iter()
            .map(|tl| {
                tl.segments
                    .iter()
                    .map(|seg| {
                        let (mfg, mbg) = app.style(seg.id);
                        let fg = mfg.unwrap_or(Color::Reset);
                        let bg = mbg.unwrap_or(Color::Reset);
                        (fg, bg, seg.text.as_ref(), seg.len)
                    }).collect()
            }).collect();
        Frame::new(size, image, focus)
    }
}

impl<N> fmt::Debug for TextBlock<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {{ lines: {:?}, size: {:?}, cursors: {:?} }}",
            self.lines, self.size, self.cursors
        )
    }
}

pub type FrameLine<'a> = Vec<(Color, Color, &'a str, usize)>;

pub struct Frame<'a> {
    pub size: Size,
    pub image: Vec<FrameLine<'a>>,
    pub focus: Option<Pos>,
}

impl<'a> Frame<'a> {
    pub fn new(size: Size, image: Vec<FrameLine<'a>>, focus: Option<Pos>) -> Self {
        Self { size, image, focus }
    }
}
