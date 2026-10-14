use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{events::{DatabaseResponse, HistoryRequest, PopupResponse, Request, Response}, ui::{components::base::component::Component, sections::ESectionId, views::{base::view::View, popup_view::{PopupData, PopupView}}}};

pub struct PopupComponent {
    view: PopupView,
    active: bool
}

impl PopupComponent {
    pub fn is_ready_to_show(&self) -> bool {
        self.active
    }
}

impl Component for PopupComponent {
    fn new() -> Self {
        PopupComponent {
            view: PopupView::default(),
            active: false
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String> {
        Ok(Request::Nothing)
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Popup(popup_response) => {
                match popup_response {
                    PopupResponse::Show(data) => {
                        self.view.set_data(&data);
                        self.active = true;
                    },
                    _ => {},
                }
            },
            _ => {
            }
        }
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        self.view.render_frame(frame, area);
    }
}
