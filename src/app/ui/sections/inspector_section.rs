use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Paragraph, Widget, Wrap,
};
use ratatui::symbols;
use tracing::warn;

use crate::app::base::{Target, TargetInfo};
use crate::app::events::{DatabaseResponse, Request};
use crate::app::ui::sections::{ESectionId, Inspectable};
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::inspector_view::{InspectorData, InspectorView};
use crate::app::ui::views::metric_bar_view::{MetricBarData, MetricBarView};
use crate::app::ui::views::metric_view::{MetricView, MetricData};
use crate::app::{events::Response, ui::{sections::base::section::Section, styles, views::list_view::ListView}};

pub struct InspectorSection {
    view: InspectorView,
    target_info: Option<TargetInfo>
}

impl InspectorSection {
    pub fn new() -> Self {
        InspectorSection {
            view: InspectorView::new(),
            target_info: None
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

    fn handle_response(&mut self, response: &Response) {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Info(id, info) => {
                        self.target_info = Some(info.clone());
                    },
                    _ => {
                    },
                }
            },
            _ => {
            },
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match &self.target_info {
            Some(info) => {
                match InspectorData::try_from(info.clone()) {
                    Ok(data) => {
                        self.view.render_data(area, buf, &data);
                    },
                    Err(_) => {},
                }
            },
            None => {
                self.view.render(area, buf);
            },
        }
    }
}
