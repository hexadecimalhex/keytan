use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::ui::widgets::notes::page::{NotePage, NotePageState};

use super::Screen;

#[derive(Default)]
pub struct Home<'a> {
    /// A list of loaded pages.
    pub pages: Vec<NotePage<'a>>,
    /// The index of the currently selected page, if any.
    pub selected_page: Option<u16>,
}

impl<'a> Home<'a> {
    /// Gets the currently selected page, if any.
    pub fn get_selected_page(&self) -> Option<&NotePage<'a>> {
        self.pages.get(self.selected_page? as usize)
    }
}

impl Screen for Home<'_> {
    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let mut note_page_state = NotePageState::default();

        if let Some(page) = self.get_selected_page() {
            frame.render_stateful_widget(page.clone(), area, &mut note_page_state);
        }
    }

    fn handle_key(&mut self, _: KeyCode) {}
}
