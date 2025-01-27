use crossterm::event::KeyCode;
use ratatui::{
    layout::{Margin, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::Screen;

#[derive(Default)]
pub struct Login {
    instance: String,
    username: String,
    password: String,
    selected: Field,
}

pub enum Message {
    /// Selects the next field.
    NextField,
    /// Selects the previous field.
    PrevField,
    /// Deletes one character in the selected field.
    Delete,
}

impl Login {
    fn message(&mut self, message: Message) {
        match message {
            Message::NextField => self.selected = self.selected.next(),
            Message::PrevField => self.selected = self.selected.prev(),
            Message::Delete => self.delete_char(),
        }
    }
    /// Deletes the last character of the currently selected field.
    /// Does nothing if the field is empty.
    pub fn delete_char(&mut self) {
        let text = match self.selected {
            Field::Instance => &mut self.instance,
            Field::Username => &mut self.username,
            Field::Password => &mut self.password,
        };

        if !text.is_empty() {
            let mut chars = text.chars();
            chars.next_back();
            let text: String = chars.collect();

            match self.selected {
                Field::Instance => self.instance = text,
                Field::Username => self.username = text,
                Field::Password => self.password = text,
            }
        }
    }

    fn draw_form(&self, frame: &mut Frame) {
        let area = frame.area();

        let form_rect = area.inner(Margin {
            horizontal: area.width / 4,
            vertical: area.height / 3,
        });
        let form_frame = Block::new()
            .borders(Borders::TOP)
            .border_type(BorderType::Plain)
            .title_top("Login");

        let mut line = form_rect.y + 1;

        let instance_label_rect = Rect::new(form_rect.x, line, form_rect.width, 1);
        let instance_label_text = Text::from(if self.selected == Field::Instance {
            "> Instance"
        } else {
            "Instance"
        });
        let instance_text_rect = Rect::new(form_rect.x, line + 1, form_rect.width, 1);
        let instance_text = Text::from(self.instance.clone());
        line += 3;
        let username_label_rect = Rect::new(form_rect.x, line, form_rect.width, 1);
        let username_label_text = Text::from(if self.selected == Field::Username {
            "> Username"
        } else {
            "Username"
        });
        let username_text_rect = Rect::new(form_rect.x, line + 1, form_rect.width, 1);
        let username_text = Text::from(self.username.clone());
        line += 3;
        let password_label_rect = Rect::new(form_rect.x, line, form_rect.width, 1);
        let password_label_text = Text::from(if self.selected == Field::Password {
            "> Password"
        } else {
            "Password"
        });
        let password_text_rect = Rect::new(form_rect.x, line + 1, form_rect.width, 1);
        let password_text = Text::from(
            self.password
                .clone()
                .chars()
                .map(|_| '*')
                .collect::<String>(),
        );

        frame.render_widget(form_frame, form_rect);
        frame.render_widget(instance_label_text, instance_label_rect);
        frame.render_widget(instance_text, instance_text_rect);
        frame.render_widget(username_label_text, username_label_rect);
        frame.render_widget(username_text, username_text_rect);
        frame.render_widget(password_label_text, password_label_rect);
        frame.render_widget(password_text, password_text_rect);
    }
}

impl Screen for Login {
    fn view(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let screen_frame = Block::new()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_type(BorderType::Plain)
            .title_bottom("Confirm (Enter) / Next (Tab) / Back (Shift-Tab) / Exit (Esc)");

        frame.render_widget(screen_frame, area);
        self.draw_form(frame);
    }
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => match self.selected {
                Field::Instance => self.instance.push(c),
                Field::Username => self.username.push(c),
                Field::Password => self.password.push(c),
            },
            KeyCode::Backspace => self.message(Message::Delete),
            KeyCode::Tab => self.message(Message::NextField),
            KeyCode::BackTab => self.message(Message::PrevField),
            _ => (),
        }
    }
}

#[derive(PartialEq)]
enum Field {
    Instance,
    Username,
    Password,
}

impl Field {
    fn next(&self) -> Self {
        match self {
            Self::Instance => Self::Username,
            Self::Username => Self::Password,
            Self::Password => Self::Instance,
        }
    }
    fn prev(&self) -> Self {
        match self {
            Self::Instance => Self::Password,
            Self::Username => Self::Instance,
            Self::Password => Self::Username,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::Instance
    }
}
