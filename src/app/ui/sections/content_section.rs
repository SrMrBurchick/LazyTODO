use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{Frame, symbols};
use tracing::info;

use crate::app::base::Target;
use crate::app::base::projects::{Project, projects_to_widget_list};
use crate::app::base::tasks::{SubTask, Task, sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::event_listener::EventsListener;
use crate::app::events::{DatabaseResponse, Request};
use crate::app::managers::context_manager::ContextManager;
use crate::app::ui::sections::{ESectionId, ESubsectionId};
use crate::app::ui::views::base::view::{View, ViewStateController};
use crate::app::ui::views::list_view::ListViewItem;
use crate::app::{events::Response, ui::{sections::base::section::Section, styles, views::list_view::ListView}};

pub struct ContentSection {
    view: ListView,
    sub_section: Option<ESubsectionId>,
    active_target: Option<Target>,
    projects: Vec<Project>,
    tasks: Vec<Task>,
    subtasks: Vec<SubTask>
}

impl ContentSection {
    pub fn new() -> Self {
        ContentSection {
            view: ListView::new("Content"),
            sub_section: None,
            active_target: None,
            projects: vec![],
            tasks: vec![],
            subtasks: vec![]
        }
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

    fn handle_response(&mut self, response: &Response) -> Result<Request, String> {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Tasks(project_id, _) => {
                        self.active_target = Some(Target::Task);
                        self.sub_section = Some(ESubsectionId::Tasks(project_id.clone()));
                    },
                    DatabaseResponse::SubTasks(task_id, _) => {
                        self.active_target = Some(Target::SubTask);
                        self.sub_section = Some(ESubsectionId::SubTasks(task_id.clone()));
                    },
                    DatabaseResponse::Projects(_) => {
                        self.active_target = Some(Target::Project);
                        self.sub_section = Some(ESubsectionId::Projects);
                    },
                    DatabaseResponse::All(_, _) => {
                        self.active_target = Some(Target::All);
                        self.sub_section = None;
                    },
                    _ => {
                    },
                }
            }
            _ => {
            },
        }

        Ok(Request::Nothing)
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        match key.code {
            _ => {
                self.view.handle_key(key)
            },
        }
    }

    fn mark_selected(&mut self, is_selected: bool) {
        self.view.mark_selected(is_selected);
    }

    fn render(&mut self, area: Rect, frame: &mut Frame) {
        // content
        // match &self.active_target {
        //     Some(target) => {
		// 		self.view.set_items(ContextManager::get().get_target_list_view(target.clone()));
        //     },
        //     None => {
        //     },
        // }

        self.view.set_items(ContextManager::get().get_active_taret_list_view());
        self.view.render(area, frame.buffer_mut());
    }

}
