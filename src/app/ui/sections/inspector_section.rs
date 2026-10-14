use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::app::events::{DatabaseResponse, Request};
use crate::app::ui::sections::ESectionId;
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::inspector_view::{InspectorData, InspectorView};
use crate::app::{events::Response, ui::{sections::base::section::Section}};

pub struct InspectorSection {
    view: InspectorView
}

impl InspectorSection {
    pub fn new() -> Self {
        InspectorSection {
            view: InspectorView::new()
        }
    }
}

impl Section for InspectorSection {
    fn initialize(&mut self) {
    }

    fn reset(&mut self) {
        self.view.clean();
    }

    fn get_id(&self) -> Option<ESectionId> {
        Some(ESectionId::Inspector)
    }

    fn handle_response(&mut self, response: &Response) -> Result<Request, String> {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Info(_, info) => {
                        match InspectorData::try_from(info.clone()) {
                            Ok(data) => {
                                self.view.set_data(data);
                            },
                            Err(_) => {},
                        }
                    },
                    _ => {
                    },
                }
            },
            _ => {
            },
        }

        Ok(Request::Nothing)
    }

    fn mark_selected(&mut self, is_selected: bool) {
        self.view.mark_selected(is_selected);
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, frame: &mut Frame) {
        self.view.render(area, frame.buffer_mut());
    }
}
