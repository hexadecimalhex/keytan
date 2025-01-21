use std::marker::PhantomData;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::StatefulWidget,
};

use crate::ui::widgets::feed_bar::FeedBar;

use super::page::{NotePage, NotePageState};

/// A generic, paginated feed of notes.
#[derive(Default, Clone)]
pub struct NoteFeed<'a> {
    _pd: PhantomData<&'a ()>,
}

impl NoteFeed<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn layout() -> Layout {
        Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(4), // header.
                Constraint::Fill(1),   // notes.
                Constraint::Length(1), // status.
            ],
        )
    }
}

impl<'a> StatefulWidget for NoteFeed<'a> {
    type State = NoteFeedState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Self::layout().split(area);

        let (bar, mut bar_state) = FeedBar::new_with_state();
        bar.render(layout[0], buf, &mut bar_state);
        if let Some((page, page_state)) = state.get_selected_page() {
            page.clone().render(layout[1], buf, &mut page_state.clone());
        }
    }
}

#[derive(Default)]
pub struct NoteFeedState<'a> {
    pub pages: Vec<(NotePage<'a>, NotePageState<'a>)>,
    pub selected_page_idx: Option<usize>,
}

impl<'a> NoteFeedState<'a> {
    /// Gets a mutable reference to the currently selected page, if any.
    pub fn get_selected_page_mut(&mut self) -> Option<&mut (NotePage<'a>, NotePageState<'a>)> {
        self.pages.get_mut(self.selected_page_idx?)
    }
    /// Gets a reference to the currently selected page, if any.
    pub fn get_selected_page(&self) -> Option<&(NotePage<'a>, NotePageState<'a>)> {
        self.pages.get(self.selected_page_idx?)
    }
    /// Selects page with given `idx`.
    /// This doesn't check whether the page exists or not.
    pub fn select(&mut self, idx: Option<usize>) {
        self.selected_page_idx = idx;
    }
    pub fn select_next_page(&mut self) {
        if let Some(current) = self.selected_page_idx {
            if current < self.pages.len() - 1 {
                self.selected_page_idx = Some(current + 1);
            }
        }
    }
    pub fn select_prev_page(&mut self) {
        if let Some(current) = self.selected_page_idx {
            if current == 0 {
                return;
            }

            self.selected_page_idx = Some(current - 1);
        }
    }
    pub fn select_next_note(&mut self) {
        if let Some((_, page_state)) = self.get_selected_page_mut() {
            page_state.next();
        }
    }
    pub fn select_prev_note(&mut self) {
        if let Some((_, page_state)) = self.get_selected_page_mut() {
            page_state.prev();
        }
    }
}
