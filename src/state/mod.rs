use std::process;

use crossterm::event::KeyCode;

use crate::ui::Ui;

#[derive(Default)]
pub struct KeytanState {
    pub ui: Ui,
}

impl KeytanState {
    // TODO: handle key up and other events for flexibility.
    pub fn handle_key(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            ratatui::restore();
            process::exit(0);
        }

        self.ui.handle_key(key);
    }
}
