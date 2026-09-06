use std::fmt;
use ratatui::widgets::ListItem;

use crate::app::{base::tasks::Task, ui::base::widget_list::WidgetListItem};

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
        format!("{self}")
    }

    fn render(&self) -> ListItem<'_> {
        ListItem::from(self)
    }
}
