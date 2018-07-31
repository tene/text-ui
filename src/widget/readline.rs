use input::Key;
use {Event, FullGrowthPolicy, InputEvent, RenderBackend, RenderContext, Widget};

#[derive(Debug)]
pub struct Readline {
    pub name: String,
    pub line: String,
    pub index: usize,
}

impl Readline {
    pub fn new(name: &str) -> Self {
        let line = String::new();
        let index = 0;
        let name = name.to_owned();
        Readline { name, line, index }
    }
}

impl<B> Widget<B> for Readline
where
    B: RenderBackend,
{
    fn render(&self, mut ctx: B::Context) -> B::Element {
        //ctx.line(&self.line)
        ctx.line(&format!("> {:?}", self))
    }
    fn handle_event(&mut self, event: &Event) -> (Option<Event>) {
        match event {
            Event::Input(event) => match event {
                InputEvent::Key(Key::Char('\n')) => {
                    self.index = 0;
                    let line = self.line.split_off(0);
                    Some(Event::readline(self.name.clone(), line))
                }
                InputEvent::Key(Key::Char(ch)) => {
                    self.line.insert(self.index, *ch);
                    self.index += 1;
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
}
