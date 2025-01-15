use crossterm::event::KeyCode;

use crate::ui::Ui;

#[derive(Default)]
pub struct KeytanState {
    pub ui: Ui,
    pub done: bool,
}

impl KeytanState {
    // TODO: handle key up and other events for flexibility.
    pub fn handle_input(&mut self, key: KeyCode) {
        self.ui.handle_key(key);
    }
    pub fn exit(&mut self) {
        ratatui::restore();
        self.done = true;
    }
}
