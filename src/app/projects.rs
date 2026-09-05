use std::fmt;
use crate::app::tasks::Task;

#[derive(Default)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub tasks: Vec<Task>
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
