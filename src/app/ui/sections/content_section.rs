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
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

pub struct ContentSection {
    view: ListView,
    focused: bool
}

impl ContentSection {
    pub fn new() -> Self {
        ContentSection {
            view: ListView::new("Content"),
            focused: false
        }
    }
}

impl Section for ContentSection {
    fn initialize(&mut self) {
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn reset(&mut self) {
        self.view.clean();
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
    }


    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Tasks(tasks) => {
                        self.view.set_items(tasks_to_widget_list(tasks));
                    },
                    DatabaseResponse::SubTasks(tasks) => {
                        self.view.set_items(sub_tasks_to_widget_list(tasks));
                    },
                    DatabaseResponse::Projects(projects) => {
                        self.view.set_items(projects_to_widget_list(projects));
                    }
                    DatabaseResponse::All(projects, tasks) => {
                        self.view.set_items(projects_to_widget_list(projects));
                        self.view.append_items(tasks_to_widget_list(tasks));
                    }
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

    fn get_name(&self) -> String {
        "Content".to_uppercase().to_string()
    }
}
