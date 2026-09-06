use std::fmt::{self, Display};

use ratatui::widgets::ListItem;

use crate::app::ui::base::widget_list::WidgetListItem;

#[derive(Clone)]
pub enum ETaskState {
    Todo = 0,
    InProgress = 1,
    Completed = 2
}

#[derive(Clone)]
pub struct SubTask {
    pub id: i64,
    pub parent_task_id: i64,
    pub title: String,
    pub description: String,
    pub state: ETaskState
}

#[derive(Clone)]
pub struct Task {
    pub id: i64,
    pub project_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub state: ETaskState,
    pub sub_tasks: Vec<SubTask>
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

impl WidgetListItem for Task {
    fn display(&self) -> String {
        format!("{self}")
    }

    fn render(&self) -> ListItem<'_> {
        ListItem::from(self)
    }
}
