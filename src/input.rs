use std::{sync::Arc, thread};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use parking_lot::RwLock;

use crate::state::KeytanState;

/// Handles keyboard input and controls.
pub struct InputControl {
    state: Arc<RwLock<KeytanState>>,
}

impl InputControl {
    pub fn new(state: Arc<RwLock<KeytanState>>) -> Self {
        Self { state }
    }
    /// Listens to and handles keyboard inputs in a separate thread.
    pub fn listen(&mut self) {
        let state = self.state.clone();

        thread::spawn(move || loop {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    let mut state = state.write();

                    if key.code == KeyCode::Esc {
                        state.exit();
                        return;
                    }

                    state.ui.handle_key(key.code);
                }
            }
        });
    }
}
