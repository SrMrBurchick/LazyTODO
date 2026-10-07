use std::fmt::{self, Display};

use crossterm::event::KeyCode;
use ratatui::widgets::ListItem;
use tracing::info;

use crate::app::{base::Target, events::{DatabaseRequest, DatabaseResponse, Request, SelectionRequest, event_listener::EventsListener}, ui::{views::list_view::ListViewItem}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ETaskState {
    Todo = 0,
    InProgress = 1,
    Completed = 2
}

#[derive(Clone, Debug)]
pub struct SubTask {
    pub id: i64,
    pub parent_task_id: i64,
    pub title: String,
    pub description: String,
    pub state: ETaskState
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: i64,
    pub project_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub state: ETaskState,
    pub sub_tasks: Vec<SubTask>
}

impl EventsListener for SubTask {
    type WorkerResponse = DatabaseResponse;
    fn handle_response(&mut self, response: &Self::WorkerResponse) -> Result<Request, String> {
        match response {
            DatabaseResponse::Updated(target, state, id) => {
                if *target == Target::SubTask {
                    match id {
                        Some(subtask_id) => {
                            if self.id == *subtask_id {
                                self.state = state.clone();
                            }
                        },
                        None => {},
                    }
                }
            }
            _ => {},
        }
        Ok(Request::Nothing)
    }
}


impl Default for SubTask {
    fn default() -> Self {
        SubTask {
            id: -1,
            parent_task_id: -1,
            title: String::default(),
            description: String::default(),
            state: ETaskState::Todo
        }
    }
}

impl fmt::Display for SubTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "[{}]({}) {} - {} {}",
            self.id,
            self.parent_task_id,
            self.title,
            self.description,
            self.state
        )
    }
}

impl EventsListener for Task {
    type WorkerResponse = DatabaseResponse;

    fn handle_response(&mut self, response: &Self::WorkerResponse) -> Result<Request, String> {
        match response {
            DatabaseResponse::Updated(target, state, id) => {
                if *target == Target::Task {
                    match id {
                        Some(task_id) => {
                            if self.id == *task_id {
                                self.state = state.clone();
                            }
                        },
                        None => {},
                    }
                }
            }
            _ => {},
        }
        Ok(Request::Nothing)
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: -1,
            project_id: None,
            title: String::default(),
            description: String::default(),
            state: ETaskState::Todo,
            sub_tasks: vec![]
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f, "[{}] {} - {} {}",
            self.id,
            self.title,
            self.description,
            self.state
        )?;

        for sub_task in &self.sub_tasks {
            writeln!(f, "       {}", sub_task)?;
        }

        Ok(())
    }
}

impl Display for ETaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ETaskState::Todo => write!(f, "TODO"),
            ETaskState::InProgress => write!(f, "InProgress"),
            ETaskState::Completed => write!(f, "Completed"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl TryFrom<i64> for ETaskState {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ETaskState::Todo),
            1 => Ok(ETaskState::InProgress),
            2 => Ok(ETaskState::Completed),
            _ => Err(format!("Invalid task state: {}", value))
        }
    }
}

impl ListViewItem for SubTask {
    fn display(&self) -> String {
        format!("{}", self.title)
    }

    fn render(&self) -> ListItem<'_> {
        info!("Render subtask {self}");
        ListItem::from(self)
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match key.code {
            KeyCode::Char('n') => {
                match ETaskState::try_from(self.state.clone() as i64 + 1) {
                    Ok(new_state) => {
                        return Ok(Request::Database(DatabaseRequest::UpdateState(Target::SubTask, new_state, self.id)));
                    },
                    Err(_) => {},
                }
            }
            KeyCode::Char('p') => {
                match ETaskState::try_from(self.state.clone() as i64 - 1) {
                    Ok(new_state) => {
                        return Ok(Request::Database(DatabaseRequest::UpdateState(Target::SubTask, new_state, self.id)));
                    },
                    Err(_) => {},
                }
            }
            _ => {
            }
        }

        Ok(Request::Nothing)
    }

    fn mark_selected(&self) -> Result<Request, String> {
        Ok(Request::Database(DatabaseRequest::GetInfo(Target::SubTask, Some(self.id))))
    }

}

impl ListViewItem for Task {
    fn display(&self) -> String {
        format!("{}", self.title)
    }

    fn render(&self) -> ListItem<'_> {
        ListItem::from(self)
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match key.code {
            KeyCode::Enter => {
                if !self.sub_tasks.is_empty() {
                    return Ok(Request::Database(DatabaseRequest::Get(Target::SubTask, Some(self.id))));
                }
            }
            // KeyCode::Left => {
            KeyCode::Char('n') => {
                match ETaskState::try_from(self.state.clone() as i64 + 1) {
                    Ok(new_state) => {
                        if self.sub_tasks.is_empty() {
                            return Ok(Request::Database(DatabaseRequest::UpdateState(Target::Task, new_state, self.id)));
                        }
                    },
                    Err(_) => {},
                }
            }
            // KeyCode::Right => {
            KeyCode::Char('p') => {
                match ETaskState::try_from(self.state.clone() as i64 - 1) {
                    Ok(new_state) => {
                        if self.sub_tasks.is_empty() {
                            return Ok(Request::Database(DatabaseRequest::UpdateState(Target::Task, new_state, self.id)));
                        }
                    },
                    Err(_) => {},
                }
            }

            _ => {
            }

        }

        Ok(Request::Nothing)
    }

    fn mark_selected(&self) -> Result<Request, String> {
        Ok(Request::Database(DatabaseRequest::GetInfo(Target::Task, Some(self.id))))
    }
}

pub fn tasks_to_widget_list(tasks: &Vec<Task>) -> Vec<Box<dyn ListViewItem>> {
    tasks.into_iter().map(|task| Box::new(task.clone()) as Box<dyn ListViewItem>).collect()
}

pub fn sub_tasks_to_widget_list(items: &Vec<SubTask>) -> Vec<Box<dyn ListViewItem>> {
    items.into_iter().map(|item| Box::new(item.clone()) as Box<dyn ListViewItem>).collect()
}
