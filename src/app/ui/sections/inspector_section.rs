use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::symbols;

use crate::app::base::projects::projects_to_widget_list;
use crate::app::base::tasks::{sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::{DatabaseResponse, Request};
use crate::app::ui::sections::ESectionId;
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

pub struct InspectorSection {
    view: ListView,
}

impl InspectorSection {
    pub fn new() -> Self {
        InspectorSection {
            view: ListView::new("Inspector"),
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
                    _ => {},
                }
            }
            _ => {},
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // content
        self.view.render(area, buf);
    }
}
