use crate::app::base::tasks::Task;

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: vec![]
        }
    }
}
