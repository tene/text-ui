use input::Key;
use std::fmt;
use {
    shared, AppEvent, FullGrowthPolicy, Name, Pos, RenderContext, Shared, ShouldPropagate,
    TextBlock, Widget,
};

pub enum SimpleInputEvent<'a, N>
where
    N: 'a + Name,
{
    Submitted { name: &'a N, line: &'a str },
}

struct SimpleInputInner<N>
where
    N: Name,
{
    pub name: N,
    pub line: String,
    pub index: usize,
    pub listeners: Vec<Box<Fn(&SimpleInputEvent<N>) -> bool>>,
    // XXX TODO Prompt
}

impl<N> SimpleInputInner<N>
where
    N: Name,
{
    pub fn new(name: N) -> Self {
        let line = String::new();
        let index = 0;
        let listeners = vec![];
        SimpleInputInner {
            name,
            line,
            index,
            listeners,
        }
    }
    pub fn add_listener(&mut self, l: Box<Fn(&SimpleInputEvent<N>) -> bool>) {
        self.listeners.push(l)
    }
    fn submit(&mut self) {
        self.index = 0;
        let line = &self.line.split_off(0);
        let name = &self.name;
        let event = SimpleInputEvent::Submitted { name, line };
        self.listeners.retain(|l| l(&event));
    }
    pub fn handle_key(&mut self, key: Key) -> ShouldPropagate {
        use ShouldPropagate::*;
        match key {
            Key::Char('\n') => {
                self.submit();
                Stop
            }
            Key::Char(ch) => {
                self.line.insert(self.index, ch);
                self.index += 1;
                Stop
            }
            Key::Esc => Continue,
            _ => Continue,
        }
    }
    pub fn set_index(&mut self, new_idx: usize) {
        self.index = std::cmp::min(new_idx, self.line.len());
    }
}

impl<N> fmt::Debug for SimpleInputInner<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SimpleInput")
            .field("name", &self.name)
            .field("index", &self.index)
            .field("line", &self.line)
            .finish()
    }
}

pub struct SimpleInput<N>
where
    N: Name,
{
    inner: Shared<SimpleInputInner<N>>,
}

impl<N> SimpleInput<N>
where
    N: Name,
{
    pub fn new(name: N) -> Self {
        let inner = shared(SimpleInputInner::new(name));
        SimpleInput { inner }
    }
    pub fn add_listener(self, l: Box<Fn(&SimpleInputEvent<N>) -> bool>) -> Self {
        self.inner.write().unwrap().add_listener(l);
        self
    }
}

impl<N> fmt::Debug for SimpleInput<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.read().unwrap().fmt(f)
    }
}

impl<N> Widget<N> for SimpleInput<N>
where
    N: 'static + Name,
{
    fn name(&self) -> Option<N> {
        Some(self.inner.read().unwrap().name)
    }
    fn render(&self, ctx: RenderContext<N>) -> TextBlock<N> {
        let inner = self.inner.clone();
        let inner2 = inner.clone();
        let name = inner.read().unwrap().name;
        let line = inner.read().unwrap().line.to_string();
        let index = inner.read().unwrap().index;
        ctx.with_bound(ctx.bound().constrain_height(1))
            .clip_lines("Buffer", vec![line])
            .add_key_handler(
                Some(name),
                Box::new(move |_ctx, k| inner.write().unwrap().handle_key(k)),
            ).add_mouse_handler(
                Some(name),
                Box::new(move |ctx, pos, _m| {
                    let _ = ctx.send_event(AppEvent::SetFocus(name));
                    inner2.write().unwrap().set_index(pos.col);
                    ShouldPropagate::Stop
                }),
            ).add_cursor(name, Pos::new(index, 0))
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
    fn widget_type(&self) -> &'static str {
        "SimpleInput"
    }
}
