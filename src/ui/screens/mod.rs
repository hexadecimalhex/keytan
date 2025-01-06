use crossterm::event::KeyCode;
use ratatui::Frame;

pub mod home;
pub mod login;

/// A full screen view.
pub trait Screen {
    /// Draws directly on a frame.
    fn view(&self, frame: &mut Frame);
    /// Handles keydown.
    fn handle_key(&mut self, key: KeyCode);
}
