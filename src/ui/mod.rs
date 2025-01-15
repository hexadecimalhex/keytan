use std::{sync::Arc, thread};

use crossterm::event::KeyCode;
use parking_lot::RwLock;
use ratatui::DefaultTerminal;
use screens::{home::Home, Screen};

pub mod screens;
pub mod widgets;

pub struct Ui {
    /// A handle to `ratatui`'s terminal for rendering.
    pub terminal: Arc<RwLock<DefaultTerminal>>,
    /// The screen to render.
    pub current_screen: Option<Arc<RwLock<Box<dyn Screen + Send + Sync>>>>,
}

impl Ui {
    /// Draws the current screen's contents.
    pub fn draw(&mut self) {
        let terminal = self.terminal.clone();
        let screen = self.current_screen.clone();

        thread::spawn(move || loop {
            let screen = screen.clone();
            terminal
                .write()
                .draw(|frame| {
                    if let Some(screen) = screen {
                        screen.read().view(frame);
                    }
                })
                .ok();
        });
    }

    /// Handles a key press.
    pub fn handle_key(&mut self, key: KeyCode) {
        if let Some(screen) = &mut self.current_screen {
            screen.write().handle_key(key);
        }
    }
}

impl Default for Ui {
    fn default() -> Self {
        let terminal = Arc::new(RwLock::new(ratatui::init()));

        Self {
            terminal,
            current_screen: Some(Arc::new(RwLock::new(Box::new(Home::make_dummy())))),
        }
    }
}
