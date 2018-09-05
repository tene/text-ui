use super::consts;
use super::edit::Editor;
use super::history::Direction;
use super::keymap::{Anchor, At, Cmd, InputState, Movement, Word};
use super::kill_ring::Mode;
use super::line_buffer::WordAction;
use super::state::State;

pub fn _process_char(s: &mut State, editor: &mut Editor, ch: char, input_state: &mut InputState) {
    let cmd = input_state.next_cmd(consts::_char_to_key_press(ch), s, true);
    process_command(s, editor, cmd, input_state)
}

pub fn process_command(s: &mut State, editor: &mut Editor, cmd: Cmd, input_state: &mut InputState) {
    if cmd.should_reset_kill_ring() {
        editor.reset_kill_ring();
    }

    match cmd {
        Cmd::SelfInsert(n, c) => s.edit_insert(c, n),
        Cmd::Insert(n, text) => s.edit_yank(&input_state, &text, Anchor::Before, n),
        Cmd::Move(Movement::BeginningOfLine) => {
            // Move to the beginning of line.
            s.edit_move_home()
        }
        Cmd::Move(Movement::ViFirstPrint) => {
            s.edit_move_home();
            s.edit_move_to_next_word(At::Start, Word::Big, 1)
        }
        Cmd::Move(Movement::BackwardChar(n)) => {
            // Move back a character.
            s.edit_move_backward(n)
        }
        Cmd::Kill(Movement::ForwardChar(n)) => {
            // Delete (forward) one character at point.
            s.edit_delete(n)
        }
        Cmd::Replace(n, c) => {
            s.edit_replace_char(c, n);
        }
        Cmd::Overwrite(c) => {
            s.edit_overwrite_char(c);
        }
        Cmd::Move(Movement::EndOfLine) => {
            // Move to the end of line.
            s.edit_move_end()
        }
        Cmd::Move(Movement::ForwardChar(n)) => {
            // Move forward a character.
            s.edit_move_forward(n)
        }
        Cmd::Kill(Movement::BackwardChar(n)) => {
            // Delete one character backward.
            s.edit_backspace(n)
        }
        Cmd::Kill(Movement::EndOfLine) => {
            // Kill the text from point to the end of the line.
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_kill_line();
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::Kill(Movement::WholeLine) => {
            s.edit_move_home();
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_kill_line();
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::NextHistory => {
            // Fetch the next command from the history list.
            s.edit_history_next(&editor.history, false)
        }
        Cmd::PreviousHistory => {
            // Fetch the previous command from the history list.
            s.edit_history_next(&editor.history, true)
        }
        Cmd::HistorySearchBackward => s.edit_history_search(&editor.history, Direction::Reverse),
        Cmd::HistorySearchForward => s.edit_history_search(&editor.history, Direction::Forward),
        Cmd::TransposeChars => {
            // Exchange the char before cursor with the character at cursor.
            s.edit_transpose_chars()
        }
        Cmd::Kill(Movement::BeginningOfLine) => {
            // Kill backward from point to the beginning of the line.
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_discard_line();
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::Yank(n, anchor) => {
            // retrieve (yank) last item killed
            if let Some(text) = editor.kill_ring.borrow_mut().yank() {
                s.edit_yank(&input_state, text, anchor, n)
            }
        }
        Cmd::ViYankTo(mvt) => if let Some(text) = s.line.copy(mvt) {
            editor.kill_ring.borrow_mut().kill(&text, Mode::Append)
        },
        // TODO CTRL-_ // undo
        Cmd::AcceptLine => {
            // Accept the line regardless of where the cursor is.
            s.edit_move_end();
        }
        Cmd::Kill(Movement::BackwardWord(n, word_def)) => {
            // kill one word backward (until start of word)
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_delete_prev_word(word_def, n);
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::BeginningOfHistory => {
            // move to first entry in history
            s.edit_history(&editor.history, true)
        }
        Cmd::EndOfHistory => {
            // move to last entry in history
            s.edit_history(&editor.history, false)
        }
        Cmd::Move(Movement::BackwardWord(n, word_def)) => {
            // move backwards one word
            s.edit_move_to_prev_word(word_def, n)
        }
        Cmd::CapitalizeWord => {
            // capitalize word after point
            s.edit_word(WordAction::CAPITALIZE)
        }
        Cmd::Kill(Movement::ForwardWord(n, at, word_def)) => {
            // kill one word forward (until start/end of word)
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_delete_word(at, word_def, n);
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::Move(Movement::ForwardWord(n, at, word_def)) => {
            // move forwards one word
            s.edit_move_to_next_word(at, word_def, n)
        }
        Cmd::DowncaseWord => {
            // lowercase word after point
            s.edit_word(WordAction::LOWERCASE)
        }
        Cmd::TransposeWords(n) => {
            // transpose words
            s.edit_transpose_words(n)
        }
        Cmd::UpcaseWord => {
            // uppercase word after point
            s.edit_word(WordAction::UPPERCASE)
        }
        Cmd::YankPop => {
            // yank-pop
            if let Some((yank_size, text)) = editor.kill_ring.borrow_mut().yank_pop() {
                s.edit_yank_pop(yank_size, text)
            }
        }
        Cmd::Move(Movement::ViCharSearch(n, cs)) => s.edit_move_to(cs, n),
        Cmd::Kill(Movement::ViCharSearch(n, cs)) => {
            editor.kill_ring.borrow_mut().start_killing();
            s.edit_delete_to(cs, n);
            editor.kill_ring.borrow_mut().stop_killing();
        }
        Cmd::Undo(n) => {
            s.line.remove_change_listener();
            if s.changes.borrow_mut().undo(&mut s.line, n) {
                s.refresh();
            }
            s.line.set_change_listener(s.changes.clone());
        }
        Cmd::Noop => {}
        _ => {
            // Ignore the character typed.
        }
    }
}
