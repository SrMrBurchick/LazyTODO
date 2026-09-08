use std::fmt;
use crossterm::event::KeyCode;
use ratatui::widgets::ListItem;

use crate::app::{base::{Target, tasks::Task}, events::{DatabaseRequest, Request}, ui::base::widget_list::WidgetListItem};

#[derive(Default, Clone)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub tasks: Vec<Task>
}

impl Project {
    fn request() {
        //
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f, "[{}] {} - {}",
            self.id, self.title, self.description
        )?;

        for task in &self.tasks {
            writeln!(f, "   {}", task)?;
        }

        Ok(())
    }
}

impl WidgetListItem for Project {
    fn display(&self) -> String {
        format!("{}", self.title)
    }

    fn render(&self) -> ListItem<'_> {
        ListItem::from(self)
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match key.code {
            KeyCode::Enter => {
                Ok(Request::Database(DatabaseRequest::Get(Target::Task, Some(self.id))))
            }
            _ => {
                Ok(Request::Nothing)
            }
        }
    }
}

pub fn projects_to_widget_list(items: Vec<Project>) -> Vec<Box<dyn WidgetListItem>> {
    items.into_iter().map(|item| Box::new(item) as Box<dyn WidgetListItem>).collect()
}
