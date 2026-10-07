use std::fmt;
use crossterm::event::KeyCode;
use ratatui::widgets::ListItem;

use crate::app::{
    base::{Target, tasks::Task}, events::{DatabaseRequest, DatabaseResponse, Request, SelectionRequest, event_listener::EventsListener}, ui::views::list_view::ListViewItem
};

#[derive(Default, Debug, Clone)]
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

impl ListViewItem for Project {
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

    fn mark_selected(&self) -> Result<Request, String> {
        Ok(Request::Database(DatabaseRequest::GetInfo(Target::Project, Some(self.id))))
    }
}

impl EventsListener for Project {
    type WorkerResponse = DatabaseResponse;
    fn handle_response(&mut self, response: &Self::WorkerResponse) -> Result<Request, String> {
        Ok(Request::Nothing)
    }
}

pub fn projects_to_widget_list(items: &Vec<Project>) -> Vec<Box<dyn ListViewItem>> {
    items.into_iter().map(|item| Box::new(item.clone()) as Box<dyn ListViewItem>).collect()
}
