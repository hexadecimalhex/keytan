use crossterm::event::KeyCode;
use ratatui::{
    layout::{self, Constraint, Layout},
    Frame,
};

use crate::ui::widgets::{
    feed_bar::FeedBar,
    notes::{
        page::{NotePage, NotePageState},
        Note, User,
    },
};

use super::Screen;

#[derive(Default)]
pub struct Home<'a> {
    /// A list of loaded pages and their states.
    pub pages: Vec<(NotePage<'a>, NotePageState<'a>)>,
    /// The index of the currently selected page, if any.
    pub selected_page: Option<usize>,
    /// Whether to jump down on the next `g` key.
    pub waiting_start: bool,
}

pub enum Message {
    /// Selects the next or previous note in the current page.
    SetNote(Direction),
    /// Goes to the next of previous page.
    SetPage(Direction),
    /// Selects the first note.
    JumpToStart,
    /// Selects the last note.
    JumpToEnd,
    /// Signals to select the last note when `g` is pressed again.
    WaitStart,
}

pub enum Direction {
    Next,
    Previous,
}

impl<'a> Home<'a> {
    fn message(&mut self, message: Message) {
        match message {
            Message::SetNote(Direction::Next) => {
                self.next_selected();
                self.waiting_start = false;
            }
            Message::SetNote(Direction::Previous) => {
                self.prev_selected();
                self.waiting_start = false;
            }
            Message::SetPage(Direction::Next) => self.next_page(),
            Message::SetPage(Direction::Previous) => self.prev_page(),
            Message::JumpToStart => {
                if let Some((_, state)) = self.get_selected_page_mut() {
                    state.select(0);
                }
            }
            Message::JumpToEnd => {
                if let Some((_, state)) = self.get_selected_page_mut() {
                    if !state.notes.is_empty() {
                        state.select(state.notes.len() - 1)
                    }
                    self.waiting_start = false;
                }
            }
            Message::WaitStart => self.waiting_start = true,
        }
    }
    // TODO: separate note feed into a widget.
    /// Gets a mutable reference to the currently selected page, if any.
    pub fn get_selected_page_mut(&mut self) -> Option<&mut (NotePage<'a>, NotePageState<'a>)> {
        self.pages.get_mut(self.selected_page?)
    }
    /// Gets a reference to the currently selected page, if any.
    pub fn get_selected_page(&self) -> Option<&(NotePage<'a>, NotePageState<'a>)> {
        self.pages.get(self.selected_page?)
    }
    /// Go forward one page, if any.
    pub fn next_page(&mut self) {
        let Some(selected_page) = self.selected_page else {
            return;
        };

        if self.pages.get(selected_page + 1).is_some() {
            self.selected_page = Some(selected_page + 1);
        }
    }
    /// Go back one page, if any.
    pub fn prev_page(&mut self) {
        let Some(selected_page) = self.selected_page else {
            return;
        };

        if selected_page == 0 {
            return;
        }

        if self.pages.get(selected_page - 1).is_some() {
            self.selected_page = Some(selected_page - 1);
        }
    }
    /// Go down one note in the current page, if any.
    pub fn next_selected(&mut self) {
        let Some(selected) = self.selected_page else {
            return;
        };

        if let Some((_, state)) = self.pages.get_mut(selected) {
            state.next();
        }
    }
    /// Go up one note in the current page, if any.
    pub fn prev_selected(&mut self) {
        let Some(selected) = self.selected_page else {
            return;
        };

        if let Some((_, state)) = self.pages.get_mut(selected) {
            state.prev();
        }
    }
}

impl Home<'_> {
    fn layout() -> Layout {
        Layout::new(
            layout::Direction::Vertical,
            [Constraint::Length(4), Constraint::Fill(1)],
        )
    }

    pub fn make_dummy() -> Self {
        let dummy_author = User {
            name: "John Misskey".into(),
            username: "johnmisskey@misskey.io".into(),
        };
        Self {
            waiting_start: false,
            selected_page: Some(0),
            pages: vec![
                (NotePage::default(), NotePageState::new(
                        Vec::from([
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                        ])
                )),
                (NotePage::default(), NotePageState::new(vec![
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.")),
                            Note::new(dummy_author.clone(), Some("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
                ]))
            ]
        }
    }
}

impl Screen for Home<'_> {
    fn view(&self, frame: &mut Frame) {
        let layout = Home::layout().split(frame.area());

        let (bar, mut bar_state) = FeedBar::new_with_state();

        frame.render_stateful_widget(bar, layout[0], &mut bar_state);
        if let Some((page, state)) = self.get_selected_page() {
            frame.render_stateful_widget(page.clone(), layout[1], &mut state.clone());
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') => self.message(Message::SetNote(Direction::Next)),
            KeyCode::Char('k') => self.message(Message::SetNote(Direction::Previous)),
            KeyCode::Char('J') => self.message(Message::SetPage(Direction::Next)),
            KeyCode::Char('K') => self.message(Message::SetPage(Direction::Previous)),
            KeyCode::Char('G') => self.message(Message::JumpToStart),
            KeyCode::Char('g') => {
                if self.waiting_start {
                    self.message(Message::JumpToEnd);
                } else {
                    self.message(Message::WaitStart);
                }
            }
            _ => (),
        }
    }
}
