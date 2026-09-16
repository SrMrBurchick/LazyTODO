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
use tracing::info;

use crate::app::base::projects::projects_to_widget_list;
use crate::app::base::tasks::{sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::{DatabaseResponse, Request};
use crate::app::ui::sections::{ESectionId, ESubsectionId};
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

pub struct ContentSection {
    view: ListView,
    sub_section: Option<ESubsectionId>,
    prev_response: Option<DatabaseResponse>,
    prev_position: Option<usize>,
    history: Vec<(Option<usize>, Option<DatabaseResponse>)>
}

impl ContentSection {
    pub fn new() -> Self {
        ContentSection {
            view: ListView::new("Content"),
            sub_section: None,
            prev_response: None,
            prev_position: None,
            history: vec![]
        }
    }

    fn save_history(&mut self) {
        match &self.prev_response {
            Some(response) => {
                match response {
                    DatabaseResponse::SubTasks(_) => {
                        // Skip
                    }
                    _ => {
                        info!("In View Selected: {:?}", self.view.selected());
                        self.history.push((self.view.selected(), self.prev_response.clone()));
                    },
                }
            },
            None => {},
        }
    }

    fn clear_history(&mut self) {
        self.history.clear();
    }

    fn pop_from_history(&mut self) -> Option<DatabaseResponse> {
        match self.history.pop() {
            Some(response) => {
                self.prev_position = response.0;
                return response.1;
            },
            None => {},
        }
        None
    }
}

impl Section for ContentSection {
    fn initialize(&mut self) {
    }

    fn reset(&mut self) {
        self.view.clean();
    }

    fn get_id(&self) -> Option<ESectionId> {
        Some(ESectionId::Content(self.sub_section))
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Database(database_response) => {
                self.save_history();
                self.prev_response = Some(database_response.clone());
                match database_response {
                    DatabaseResponse::Tasks(tasks) => {
                        self.view.set_items(tasks_to_widget_list(tasks));
                    },
                    DatabaseResponse::SubTasks(tasks) => {
                        self.view.set_items(sub_tasks_to_widget_list(tasks));
                    },
                    DatabaseResponse::Projects(projects) => {
                        self.clear_history();
                        self.view.set_items(projects_to_widget_list(projects));
                    }
                    DatabaseResponse::All(projects, tasks) => {
                        self.clear_history();
                        self.view.set_items(projects_to_widget_list(projects));
                        self.view.append_items(tasks_to_widget_list(tasks));
                    }
                    _ => {},
                }

                info!("Restore Selected: {:?}", self.prev_position);
                self.view.select(self.prev_position);
            }
            _ => {},
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        match key.code {
            KeyCode::Esc => {
                match self.pop_from_history() {
                    Some(database_response) => {
                        self.handle_response(Response::Database(database_response));
                    },
                    None => {},
                }
                Ok(Request::Nothing)
            },
            _ => {
                self.view.handle_key(key)
            },
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // content
        self.view.render(area, buf);
    }

}
