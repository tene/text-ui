use input::Key;
use {
    shared, AppEvent, FullGrowthPolicy, Name, Pos, RenderContext, Shared, ShouldPropagate,
    TextBlock, Widget,
};

use std::fmt;

mod config;
mod consts;
mod edit;
mod history;
mod keymap;
mod kill_ring;
mod line_buffer;
mod process;
mod state;
mod undo;

use self::consts::KeyPress;
pub use self::edit::Editor;
use self::keymap::InputState;
use self::process::process_command;
pub use self::state::State;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Offset {
    pub col: usize,
    pub row: usize,
}

impl Offset {
    pub fn new(col: usize, row: usize) -> Self {
        Offset { col, row }
    }
}

pub enum ReadlineEvent<'a, N>
where
    N: 'a + Name,
{
    Submitted { name: N, line: &'a str },
}

pub struct Readline<N: Name> {
    inner: Shared<ReadlineInner<N>>,
}
struct ReadlineInner<N: Name> {
    state: State,
    editor: Editor,
    input_state: InputState,
    name: N,
    listeners: Vec<Box<Fn(&ReadlineEvent<N>) -> bool>>,
}

impl<N: Name> fmt::Debug for Readline<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = self.inner.read().unwrap();
        f.debug_struct("Readline")
            .field("name", &inner.name)
            .field("input_state", &inner.input_state)
            .field("state", &inner.state)
            .field("editor", &inner.editor)
            .finish()
    }
}

impl<N: Name> Readline<N> {
    pub fn new(name: N) -> Self {
        let mut state = State::new(0);
        let editor = Editor::new();
        let input_state = InputState::new(&editor.config, editor.custom_bindings.clone());
        state.line.set_delete_listener(editor.kill_ring.clone());
        state.line.set_change_listener(state.changes.clone());
        let listeners = vec![];
        let inner = shared(ReadlineInner {
            state,
            editor,
            input_state,
            name,
            listeners,
        });
        Self { inner }
    }
    pub fn add_listener(self, l: Box<Fn(&ReadlineEvent<N>) -> bool>) -> Self {
        self.inner.write().unwrap().add_listener(l);
        self
    }
}

impl<N: Name> ReadlineInner<N> {
    fn _update(&mut self, buf: &str, pos: usize) {
        self.state.line.update(buf, pos);
        self.state.refresh();
    }
    fn _width(&mut self, width: usize) {
        self.state.width = width;
        self.state.refresh();
    }

    fn process_keypress(&mut self, kp: KeyPress) {
        let cmd = self.input_state.next_cmd(kp, &mut self.state, true);
        process_command(
            &mut self.state,
            &mut self.editor,
            cmd,
            &mut self.input_state,
        );
    }

    fn handle_key(&mut self, key: Key) -> ShouldPropagate {
        use ShouldPropagate::*;
        match key {
            Key::Char('\n') => {
                self.submit();
                Stop
            }
            Key::Esc => Continue,
            _ => {
                self.process_key(key);
                Stop
            }
        }
    }

    fn submit(&mut self) {
        let line = self.state.finalize();
        self.editor.add_history_entry(line.clone());
        let event = ReadlineEvent::Submitted {
            name: self.name,
            line: &line,
        };
        self.listeners.retain(|l| l(&event));
    }

    fn _set_line(&mut self, text: &str) {
        self.state.set_line(text);
    }
    fn process_key(&mut self, key: Key) {
        self.process_keypress(key_to_keypress(key))
    }
    pub fn add_listener(&mut self, l: Box<Fn(&ReadlineEvent<N>) -> bool>) {
        self.listeners.push(l)
    }
}

// text-ui integration to be broken out later
fn key_to_keypress(key: Key) -> KeyPress {
    match key {
        Key::Backspace => KeyPress::Backspace,
        Key::Left => KeyPress::Left,
        Key::Right => KeyPress::Right,
        Key::Up => KeyPress::Up,
        Key::Down => KeyPress::Down,
        Key::Home => KeyPress::Home,
        Key::End => KeyPress::End,
        Key::PageUp => KeyPress::PageUp,
        Key::PageDown => KeyPress::PageDown,
        Key::Delete => KeyPress::Delete,
        Key::Insert => KeyPress::Insert,
        Key::F(u8) => KeyPress::F(u8),
        Key::Char(char) => KeyPress::Char(char),
        Key::Alt(char) => KeyPress::Meta(char.to_ascii_uppercase()),
        Key::Ctrl(char) => KeyPress::Ctrl(char.to_ascii_uppercase()),
        Key::Null => KeyPress::Null,
        Key::Esc => KeyPress::Esc,
        _ => KeyPress::UnknownEscSeq,
    }
}

impl<N: Name + 'static> Widget<N> for Readline<N> {
    fn name(&self) -> Option<N> {
        Some(self.inner.read().unwrap().name)
    }
    fn render(&self, ctx: RenderContext<N>) -> TextBlock<N> {
        let width = ctx
            .bound
            .width
            .expect("Rendering Readline without width constraint");
        let inner = self.inner.clone();
        let (lines, focus, name) = {
            let inner = inner.read().unwrap();
            let lines = inner.state.render_width(width);
            let focus = Pos::new(inner.state.cursor.col, inner.state.cursor.row);
            (lines, focus, inner.name)
        };
        ctx.clip_lines("Buffer", lines)
            .add_key_handler(
                Some(name),
                Box::new(move |_ctx, k| inner.write().unwrap().handle_key(k)),
            ).add_mouse_handler(
                Some(name),
                Box::new(move |ctx, _pos, _m| {
                    let _ = ctx.send_event(AppEvent::SetFocus(name));
                    // update index
                    ShouldPropagate::Stop
                }),
            ).add_cursor(name, focus)
        /*
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
            */
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::fixed_height()
    }
    fn widget_type(&self) -> &'static str {
        "Readline"
    }
}
