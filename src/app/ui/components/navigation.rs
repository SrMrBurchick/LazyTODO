use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{base::{TargetInfo, TargetStatusInfo}, events::{DatabaseResponse, HistoryRequest, PopupRequest, Request, Response}, ui::{components::base::component::Component, sections::{ESectionId, ESubsectionId}}};

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
                    },
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
        match (key.code, key.modifiers) {
            (KeyCode::Right, _) => {
                self.go_forward();
            },
            (KeyCode::Left, _) => {
                self.go_back();
            },
            (KeyCode::Esc, _) => {
                return Ok(Request::History(HistoryRequest::GetPrevious));
            },
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                match self.current_active_section {
                    Some(section) => {
                        match section {
                            ESectionId::Content(subsection) => {
                                match subsection {
                                    Some(id) => {
                                        match id {
                                            ESubsectionId::Projects => {
                                                return Ok(Request::Popup(PopupRequest::Create(TargetInfo::Project(None, String::default(), String::default()))));
                                            },
                                            ESubsectionId::Tasks(project_id) => {

                                            }
                                            _ => {},
                                        }
                                    },
                                    None => {},
                                }
                            }
                            _ => {},
                        }
                    },
                    None => {},
                }
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
