use std::iter::repeat;

use unicode_segmentation::UnicodeSegmentation;

use {Name, RenderBound, Size};

// XXX TODO Better name??
#[derive(Debug, Clone, Copy)]
pub struct ContentID<N: Name> {
    pub name: Option<N>,
    pub widget: &'static str,
    pub class: &'static str,
}

impl<N: Name> ContentID<N> {
    pub fn as_tuple(self) -> (Option<N>, &'static str, &'static str) {
        (self.name, self.widget, self.class)
    }
    pub fn new(name: Option<N>, widget: &'static str, class: &'static str) -> Self {
        Self {
            name,
            widget,
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
}

impl<N: Name> TextBlock<N> {
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
        let size = Size::new(width, lines.len());
        Self { lines, size }
    }
}
