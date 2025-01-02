use crossterm::event::{self, Event, KeyEventKind};
use state::KeytanState;

pub mod state;
pub mod ui;

fn main() {
    let mut app = KeytanState::default();

    loop {
        app.ui.draw();
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key.code);
            }
        }
    }
}
