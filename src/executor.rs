use std::sync::mpsc::{channel, Receiver, Sender};

use widget::BackendContext;
use {
    App, AppEvent, EventContext, InputEvent, Name, RenderBackend, RenderContext, Size, TextBlock,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Event<N: Name> {
    Input(InputEvent),
    App(AppEvent<N>), // Ugh I think this needs a better name
    Resize(Size),
}

pub struct Executor<N: Name, B: RenderBackend> {
    size: Size,
    receiver: Receiver<Event<N>>,
    sender: Sender<Event<N>>,
    be: B,
}

impl<N: Name + 'static, B: RenderBackend> Executor<N, B> {
    pub fn new(be: B) -> Self {
        let (sender, receiver) = channel();
        let bc = BackendContext::new(sender.clone());
        let be = B::new(bc);
        let size = be.size();
        Self {
            size,
            sender,
            receiver,
            be,
        }
    }
    pub fn run(&mut self, app: &mut impl App<N>, mut focus: N) {
        let input_sender = self.sender.clone();
        let event_ctx = EventContext::new(self.sender.clone());
        'outer: loop {
            let render_ctx = RenderContext::from_widget(self.size.into(), app);
            let ui: TextBlock<N> = app.render(render_ctx);
            let frame = ui.render_frame(app, Some(focus));
            self.be.paint_frame(frame);
            {
                // LOL wait until an event before doing anything this is a dumb hack
                let event = self.receiver.recv().unwrap();
                let _ = self.sender.send(event);
            }
            for event in self.receiver.try_iter() {
                match event {
                    Event::App(AppEvent::Exit) => break 'outer,
                    Event::App(AppEvent::SetFocus(f)) => focus = f,
                    Event::Resize(size) => {
                        self.size = size;
                        self.be.resize(size);
                        app.handle_resize(size);
                    }
                    Event::Input(event) => {
                        use ShouldPropagate::*;
                        match app.handle_input(&event_ctx, &event) {
                            Stop => break,
                            Continue => {}
                        };
                        match event {
                            InputEvent::Key(k) => ui.handle_key(&event_ctx, &focus, k),
                            InputEvent::Mouse(m) => ui.handle_mouse(&event_ctx, m),
                            InputEvent::Unsupported(_) => {}
                        }
                    }
                }
            }
        }
    }
}
