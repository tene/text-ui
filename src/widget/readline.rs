use input::Key;
use {
    shared, Event, FullGrowthPolicy, InputEvent, RenderBackend, RenderContext, RenderElement,
    Shared, Widget,
};

#[derive(Debug)]
struct ReadlineInner {
    pub line: String,
    pub index: usize,
}

impl ReadlineInner {
    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Input(event) => match event {
                InputEvent::Key(Key::Char('\n')) => {
                    self.index = 0;
                    let line = self.line.split_off(0);
                    true
                }
                InputEvent::Key(Key::Char(ch)) => {
                    self.line.insert(self.index, *ch);
                    self.index += 1;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Readline {
    pub name: String,
    inner: Shared<ReadlineInner>,
}

impl Readline {
    pub fn new(name: &str) -> Self {
        let line = String::new();
        let index = 0;
        let name = name.to_owned();
        let inner = shared(ReadlineInner { line, index });
        Readline { name, inner }
    }
}

impl<B> Widget<B> for Readline
where
    B: RenderBackend,
{
    fn render(&self, mut ctx: B::Context) -> B::Element {
        //ctx.line(&self.line)
        let inner = self.inner.clone();
        let mut line = ctx.line(&format!("> {:?}", self));
        line.add_input_handler(
            &self.name,
            Box::new(move |e| inner.write().unwrap().handle_event(e)),
        );
        line
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
}
