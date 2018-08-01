use input::Key;
use std::sync::mpsc::Sender;
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
    pub fn handle_event(&mut self, event: &Event) -> (bool, Option<String>) {
        match event {
            Event::Input(event) => match event {
                InputEvent::Key(Key::Char('\n')) => {
                    self.index = 0;
                    let line = self.line.split_off(0);
                    (true, Some(line))
                }
                InputEvent::Key(Key::Char(ch)) => {
                    self.line.insert(self.index, *ch);
                    self.index += 1;
                    (true, None)
                }
                InputEvent::Key(Key::Esc) => (false, None),
                _ => (false, None),
            },
            _ => (false, None),
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
        let inner = self.inner.clone();
        let sender = ctx.event_sender();
        let name = self.name.clone();
        let mut line = ctx.line(&format!("{}", inner.read().unwrap().line));
        line.add_input_handler(
            &self.name,
            Box::new(move |e| {
                let (rv, line) = inner.write().unwrap().handle_event(e);
                if let Some(line) = line {
                    let _ = sender.send(Event::readline(name.clone(), line)); // UGH this is gross RL should have its own callbacks
                };
                rv
            }),
        );
        line
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
}
