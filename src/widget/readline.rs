use input::Key;
use std::fmt;
use {
    shared, FullGrowthPolicy, InputEvent, RenderBackend, RenderContext, RenderElement, Shared,
    Widget,
};

pub enum ReadlineEvent {
    Submitted { name: String, line: String },
}

struct ReadlineInner {
    pub name: String,
    pub line: String,
    pub index: usize,
    pub listeners: Vec<Box<Fn(&ReadlineEvent) -> bool>>,
}

impl ReadlineInner {
    pub fn new(name: &str) -> Self {
        let line = String::new();
        let index = 0;
        let name = name.to_owned();
        let listeners = vec![];
        ReadlineInner {
            name,
            line,
            index,
            listeners,
        }
    }
    pub fn add_listener(&mut self, l: Box<Fn(&ReadlineEvent) -> bool>) {
        self.listeners.push(l);
    }
    fn submit(&mut self) {
        self.index = 0;
        let line = self.line.split_off(0);
        let name = self.name.clone();
        let event = ReadlineEvent::Submitted { name, line };
        self.listeners.retain(|l| l(&event));
    }
    pub fn handle_input(&mut self, event: &InputEvent) -> bool {
        match event {
            InputEvent::Key(Key::Char('\n')) => {
                self.submit();
                true
            }
            InputEvent::Key(Key::Char(ch)) => {
                self.line.insert(self.index, *ch);
                self.index += 1;
                true
            }
            InputEvent::Key(Key::Esc) => false,
            _ => false,
        }
    }
}

impl fmt::Debug for ReadlineInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Readline {{ name: {}, index: {}, line: {} }}",
            self.name, self.index, self.line
        )
    }
}

pub struct Readline {
    inner: Shared<ReadlineInner>,
}

impl Readline {
    pub fn new(name: &str) -> Self {
        let inner = shared(ReadlineInner::new(name));
        Readline { inner }
    }
    pub fn add_listener(&mut self, l: Box<Fn(&ReadlineEvent) -> bool>) {
        self.inner.write().unwrap().add_listener(l)
    }
}

impl fmt::Debug for Readline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.read().unwrap().fmt(f)
    }
}

impl<B> Widget<B> for Readline
where
    B: RenderBackend,
{
    fn render(&self, mut ctx: B::Context) -> B::Element {
        let inner = self.inner.clone();
        let name = inner.read().unwrap().name.clone();
        let mut line = ctx.line(&format!("{}", inner.read().unwrap().line));
        line.add_input_handler(
            &name,
            Box::new(move |e| inner.write().unwrap().handle_input(e)),
        );
        line
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
}
