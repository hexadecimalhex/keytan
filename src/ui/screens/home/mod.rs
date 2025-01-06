use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::ui::widgets::notes::{
    page::{NotePage, NotePageState},
    Note, User,
};

use super::Screen;

#[derive(Default)]
pub struct Home<'a> {
    /// A list of loaded pages and their states.
    pub pages: Vec<(NotePage<'a>, NotePageState<'a>)>,
    /// The index of the currently selected page, if any.
    pub selected_page: Option<usize>,
}

pub enum Message {
    /// Selects the next or previous note in the current page.
    SetNote(Direction),
    /// Goes to the next of previous page.
    SetPage(Direction),
}

pub enum Direction {
    Next,
    Previous,
}

impl<'a> Home<'a> {
    fn message(&mut self, message: Message) {
        match message {
            Message::SetNote(Direction::Next) => self.next_selected(),
            Message::SetNote(Direction::Previous) => self.prev_selected(),
            Message::SetPage(Direction::Next) => self.next_page(),
            Message::SetPage(Direction::Previous) => self.prev_page(),
        }
    }
    /// Gets the currently selected page, if any.
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
    pub fn make_dummy() -> Self {
        let dummy_author = User {
            name: "John Misskey".into(),
            username: "johnmisskey@misskey.io".into(),
        };
        Self {
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
        let area = frame.area();

        if let Some(page) = self.get_selected_page() {
            frame.render_stateful_widget(page.0.clone(), area, &mut page.1.clone());
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') => self.message(Message::SetNote(Direction::Next)),
            KeyCode::Char('k') => self.message(Message::SetNote(Direction::Previous)),
            KeyCode::Char('J') => self.message(Message::SetPage(Direction::Next)),
            KeyCode::Char('K') => self.message(Message::SetPage(Direction::Previous)),
            _ => (),
        }
    }
}
