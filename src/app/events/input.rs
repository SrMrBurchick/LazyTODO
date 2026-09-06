use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub struct InputListener {
    should_close: bool
}


impl InputListener {
    pub fn new() -> Self {
        InputListener { should_close: false }
    }

    fn listen_events(&mut self) -> io::Result<()> {
        match event::read() {
            Ok(event) => {
                match event {
                    Event::Key(key_event) => {
                        self.handle_key_event(&key_event);
                    }
                    _ => {},
                }

            },
            Err(_) => {},
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            _ => {},
        }
    }
}
