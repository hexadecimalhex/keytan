use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::ui::widgets::notes::{
    feed::{NoteFeed, NoteFeedState},
    page::{NotePage, NotePageState},
    Note, User,
};

use super::Screen;

#[derive(Default)]
pub struct Home<'a> {
    /// Whether to jump down on the next `g` key.
    pub waiting_start: bool,
    pub feed_state: NoteFeedState<'a>,
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
                self.feed_state.select_next_note();
                self.waiting_start = false;
            }
            Message::SetNote(Direction::Previous) => {
                self.feed_state.select_prev_note();
                self.waiting_start = false;
            }
            Message::SetPage(Direction::Next) => self.feed_state.select_next_page(),
            Message::SetPage(Direction::Previous) => self.feed_state.select_prev_page(),
            Message::JumpToStart => {
                if let Some((_, state)) = self.feed_state.get_selected_page_mut() {
                    state.select(0);
                }
            }
            Message::JumpToEnd => {
                if let Some((_, state)) = self.feed_state.get_selected_page_mut() {
                    if !state.notes.is_empty() {
                        state.select(state.notes.len() - 1)
                    }
                    self.waiting_start = false;
                }
            }
            Message::WaitStart => self.waiting_start = true,
        }
    }
}

impl Home<'_> {
    pub fn make_dummy() -> Self {
        let dummy_author = User {
            name: "John Misskey".into(),
            username: "johnmisskey@misskey.io".into(),
        };
        Self {
            feed_state: NoteFeedState {
                selected_page_idx: Some(0),
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
            ] },
            waiting_start: false,
        }
    }
}

impl Screen for Home<'_> {
    fn view(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let feed = NoteFeed::new();
        frame.render_stateful_widget(feed.clone(), area, &mut self.feed_state);
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
