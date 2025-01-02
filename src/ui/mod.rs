use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use screens::{login::Login, Screen};

pub mod screens;

pub struct Ui {
    /// A handle to `ratatui`'s terminal for rendering.
    pub terminal: DefaultTerminal,
    /// The screen to render.
    pub current_screen: Option<Box<dyn Screen>>,
}

impl Ui {
    /// Draws the current screen's contents.
    pub fn draw(&mut self) {
        let screen = &self.current_screen;

        self.terminal
            .draw(|frame| {
                if let Some(screen) = screen {
                    screen.draw(frame);
                }
            })
            .ok();
    }

    /// Handles a key press.
    pub fn handle_key(&mut self, key: KeyCode) {
        if let Some(screen) = &mut self.current_screen {
            screen.handle_key(key);
        }
    }
}

impl Default for Ui {
    fn default() -> Self {
        let terminal = ratatui::init();

        Self {
            terminal,
            current_screen: Some(Box::new(Login::default())),
        }
    }
}
