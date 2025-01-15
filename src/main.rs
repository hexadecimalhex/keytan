use std::{sync::Arc, thread, time::Duration};

use input::InputControl;
use parking_lot::RwLock;
use state::KeytanState;

pub mod input;
pub mod state;
pub mod ui;

fn main() {
    let app = Arc::new(RwLock::new(KeytanState::default()));
    let mut input_control = InputControl::new(app.clone());

    input_control.listen();
    app.write().ui.draw();

    while !app.read().done {
        thread::sleep(Duration::from_millis(100))
    }
}
