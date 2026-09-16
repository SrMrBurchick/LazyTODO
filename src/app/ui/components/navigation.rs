use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{events::{DatabaseResponse, Request, Response}, ui::{components::base::component::Component, sections::ESectionId}};

pub struct NavigationComponent {
    current_active_section: Option<ESectionId>
}

impl NavigationComponent {
    fn go_to(&mut self) {
    }

    fn go_back(&mut self) {
        let active_section = self.current_active_section.clone();
        match active_section {
            Some(id) => {
                match id {
                    ESectionId::Inspector => {
                        self.current_active_section = Some(ESectionId::Content(None));
                    },
                    ESectionId::Content(_) => {
                        self.current_active_section = Some(ESectionId::Spaces)
                    }
                    _ => {},
                }
            },
            None => {},
        }
    }

    fn go_forward(&mut self) {
        let active_section = self.current_active_section.clone();
        match active_section {
            Some(id) => {
                match id {
                    ESectionId::Spaces => {
                        self.current_active_section = Some(ESectionId::Content(None));
                    },
                    ESectionId::Content(_) => {
                        self.current_active_section = Some(ESectionId::Inspector)
                    }
                    _ => {},
                }
            },
            None => {},
        }
    }

    pub fn set_active_section(&mut self, section: Option<ESectionId>) {
        self.current_active_section = section;
    }

    pub fn get_active_section(&self) -> Option<ESectionId> {
        self.current_active_section
    }
}

impl Component for NavigationComponent {
    fn new() -> Self {
        NavigationComponent {
            current_active_section: None
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String> {
        match key.code {
            KeyCode::Right => {
                self.go_forward();
            },
            KeyCode::Left => {
                self.go_back();
            }
            _ => {
            }
        }

        Ok(Request::Nothing)
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            _ => {
            }
        }
    }
}
