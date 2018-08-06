use input::Key;
use std::fmt;
use {
    shared, FullGrowthPolicy, InputEvent, Name, Pos, RenderBackend, RenderElement, Shared,
    ShouldPropagate, Widget, WidgetRenderContext,
};

pub enum ReadlineEvent<'a, N>
where
    N: 'a + Name,
{
    Submitted { name: &'a N, line: &'a str },
}

struct ReadlineInner<N>
where
    N: Name,
{
    pub name: N,
    pub line: String,
    pub index: usize,
    pub listeners: Vec<Box<Fn(&ReadlineEvent<N>) -> bool>>,
}

impl<N> ReadlineInner<N>
where
    N: Name,
{
    pub fn new(name: N) -> Self {
        let line = String::new();
        let index = 0;
        let listeners = vec![];
        ReadlineInner {
            name,
            line,
            index,
            listeners,
        }
    }
    pub fn add_listener(&mut self, l: Box<Fn(&ReadlineEvent<N>) -> bool>) {
        self.listeners.push(l)
    }
    fn submit(&mut self) {
        self.index = 0;
        let line = &self.line.split_off(0);
        let name = &self.name;
        let event = ReadlineEvent::Submitted { name, line };
        self.listeners.retain(|l| l(&event));
    }
    pub fn handle_input(&mut self, event: &InputEvent) -> ShouldPropagate {
        use ShouldPropagate::*;
        match event {
            InputEvent::Key(Key::Char('\n')) => {
                self.submit();
                Stop
            }
            InputEvent::Key(Key::Char(ch)) => {
                self.line.insert(self.index, *ch);
                self.index += 1;
                Stop
            }
            InputEvent::Key(Key::Esc) => Continue,
            _ => Continue,
        }
    }
}

impl<N> fmt::Debug for ReadlineInner<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Readline {{ name: {:?}, index: {}, line: {} }}",
            self.name, self.index, self.line
        )
    }
}

pub struct Readline<N>
where
    N: Name,
{
    inner: Shared<ReadlineInner<N>>,
}

impl<N> Readline<N>
where
    N: Name,
{
    pub fn new(name: N) -> Self {
        let inner = shared(ReadlineInner::new(name));
        Readline { inner }
    }
    pub fn add_listener(self, l: Box<Fn(&ReadlineEvent<N>) -> bool>) -> Self {
        self.inner.write().unwrap().add_listener(l);
        self
    }
}

impl<N> fmt::Debug for Readline<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.read().unwrap().fmt(f)
    }
}

impl<B, N> Widget<B, N> for Readline<N>
where
    N: 'static + Name,
    B: RenderBackend<N>,
{
    fn render(&self, mut ctx: B::RenderContext) -> B::Element {
        let inner = self.inner.clone();
        let name = inner.read().unwrap().name;
        let line = inner.read().unwrap().line.to_string();
        let index = inner.read().unwrap().index;
        ctx.line(&line)
            .add_input_handler(
                Some(name),
                Box::new(move |_ctx, e| inner.write().unwrap().handle_input(e)),
            ).add_cursor(name, Pos::new(index, 0))
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
}
