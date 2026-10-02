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

use crate::app::base::Target;
use crate::app::base::projects::{Project, projects_to_widget_list};
use crate::app::base::tasks::{SubTask, Task, sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::event_listener::EventsListener;
use crate::app::events::{DatabaseResponse, Request};
use crate::app::ui::sections::{ESectionId, ESubsectionId};
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

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

    fn handle_response(&mut self, response: &Response) {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Tasks(tasks) => {
                        self.active_target = Some(Target::Task);
                        self.tasks = tasks.clone();
                    },
                    DatabaseResponse::SubTasks(tasks) => {
                        self.active_target = Some(Target::SubTask);
                        self.subtasks = tasks.clone();
                    },
                    DatabaseResponse::Projects(projects) => {
                        self.active_target = Some(Target::Project);
                        self.projects = projects.clone();
                    },
                    DatabaseResponse::All(projects, tasks) => {
                        self.active_target = Some(Target::All);
                        self.projects = projects.clone();
                        self.tasks = tasks.clone();
                    },
                    DatabaseResponse::Updated(target, _, _) => {
                        match &self.active_target {
                            Some(_) => {
                                for project in self.projects.iter_mut() {
                                    project.handle_response(database_response);
                                }
                                for task in self.tasks.iter_mut() {
                                    task.handle_response(database_response);
                                }
                                for subtask in self.subtasks.iter_mut() {
                                    subtask.handle_response(database_response);
                                }
                            },
                            None => {
                            },
                        }

                    },
                    _ => {
                        self.active_target = None;
                        self.projects.clear();
                        self.tasks.clear();
                        self.subtasks.clear();
                    },
                }
            }
            _ => {
            },
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        match key.code {
            _ => {
                self.view.handle_key(key)
            },
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // content
        match &self.active_target {
            Some(target) => {
                let mut view_data: Vec<Box<dyn WidgetListItem>> = vec![];
                match target {
                    Target::Task => {
                        view_data = tasks_to_widget_list(&self.tasks);
                    },
                    Target::SubTask => {
                        view_data = sub_tasks_to_widget_list(&self.subtasks);
                    },
                    Target::Project => {
                        view_data = projects_to_widget_list(&self.projects);
                    },
                    Target::All => {
                        view_data = projects_to_widget_list(&self.projects);
                        view_data.extend(tasks_to_widget_list(&self.tasks));
                    },
                    _ => {},
                }
                self.view.set_items(view_data);
            },
            None => {
                self.view.set_items(vec![]);
            },
        }

        self.view.render(area, buf);
    }

}
