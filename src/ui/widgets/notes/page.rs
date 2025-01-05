use std::marker::PhantomData;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use tui_widget_list::{ListBuilder, ListState, ListView};

use super::Note;

/// A scrollable list of notes.
#[derive(Default, Clone)]
pub struct NotePage<'a> {
    _pd: PhantomData<&'a ()>,
}

impl NotePage<'_> {
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }
    /// Returns a `ListView` components with this page's notes.
    fn list_view<'a>(&self, notes: &'a [Note<'_>], width: u16) -> ListView<'a, Note<'a>> {
        let builder = ListBuilder::new(move |ctx| {
            let note = notes.get(ctx.index).unwrap().clone();
            // 1 for padding and 1 for a box line, on each side (so, 4 extra lines).
            let note_height = note.text.line_count(width) + 4;

            (note, note_height as u16)
        });

        ListView::new(builder, notes.len())
    }
}

impl<'a> StatefulWidget for NotePage<'a> {
    type State = NotePageState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list_view = self.list_view(&state.notes, area.width);

        list_view.render(area, buf, &mut ListState::default());
    }
}

#[derive(Default)]
pub struct NotePageState<'a> {
    /// The notes on this page.
    pub notes: Vec<Note<'a>>,
    /// The index of the currently selected note, if any.
    pub selected: Option<u16>,
}

impl NotePageState<'_> {
    /// Sets the selected item to a given `idx`.
    /// Does nothing if out of bounds.
    pub fn select(&mut self, idx: u16) {
        self.selected = Some(idx);
    }
    /// Selects the next note.
    /// Does nothing if at the end.
    pub fn next(&mut self) {
        if let Some(idx) = self.selected {
            if self.notes.get(idx as usize + 1).is_some() {
                self.selected = Some(idx + 1);
            }
        }
    }
    /// Selects the previous note.
    /// Does nothing if at the start.
    pub fn prev(&mut self) {
        if let Some(idx) = self.selected {
            if idx == 0 {
                return;
            }

            if self.notes.get(idx as usize - 1).is_some() {
                self.selected = Some(idx - 1);
            }
        }
    }
}

impl<'a> From<&Vec<Note<'a>>> for NotePageState<'a> {
    fn from(notes: &Vec<Note<'a>>) -> Self {
        Self {
            notes: notes.clone(),
            selected: None,
        }
    }
}
