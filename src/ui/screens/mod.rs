use crossterm::event::KeyCode;
use ratatui::Frame;

pub mod login;
pub mod home;

pub trait Screen {
    fn draw(&self, frame: &mut Frame);
    fn handle_key(&mut self, key: KeyCode);
}
