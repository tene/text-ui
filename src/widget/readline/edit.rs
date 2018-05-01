use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::Path;
use std::rc::Rc;

use super::config::Config;
use super::consts::KeyPress;
use super::history::History;
use super::keymap::Cmd;
use super::kill_ring::KillRing;

pub struct Editor {
    pub history: History,
    pub kill_ring: Rc<RefCell<KillRing>>,
    pub config: Config,
    pub custom_bindings: Rc<RefCell<HashMap<KeyPress, Cmd>>>,
}

impl Editor {
    /// Create an editor with the default configuration
    pub fn new() -> Editor {
        Self::with_config(Config::default())
    }

    /// Create an editor with a specific configuration.
    pub fn with_config(config: Config) -> Editor {
        Editor {
            history: History::with_config(config),
            kill_ring: Rc::new(RefCell::new(KillRing::new(60))),
            config,
            custom_bindings: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Load the history from the specified file.
    pub fn load_history<P: AsRef<Path> + ?Sized>(&mut self, path: &P) -> Result<(), io::Error> {
        self.history.load(path)
    }
    /// Save the history in the specified file.
    pub fn save_history<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<(), io::Error> {
        self.history.save(path)
    }
    /// Add a new entry in the history.
    pub fn add_history_entry<S: AsRef<str> + Into<String>>(&mut self, line: S) -> bool {
        self.history.add(line)
    }
    /// Clear history.
    pub fn clear_history(&mut self) {
        self.history.clear()
    }
    /// Return a mutable reference to the history object.
    pub fn get_history(&mut self) -> &mut History {
        &mut self.history
    }
    /// Return an immutable reference to the history object.
    pub fn get_history_const(&self) -> &History {
        &self.history
    }

    /// Bind a sequence to a command.
    pub fn bind_sequence(&mut self, key_seq: KeyPress, cmd: Cmd) -> Option<Cmd> {
        self.custom_bindings.borrow_mut().insert(key_seq, cmd)
    }
    /// Remove a binding for the given sequence.
    pub fn unbind_sequence(&mut self, key_seq: KeyPress) -> Option<Cmd> {
        self.custom_bindings.borrow_mut().remove(&key_seq)
    }

    pub fn reset_kill_ring(&self) {
        self.kill_ring.borrow_mut().reset();
    }
}

impl fmt::Debug for Editor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Editor")
            .field("config", &self.config)
            .field("history", &self.history)
            .finish()
    }
}
