use crate::app::base::{
    tasks::Task,
    projects::Project
};

pub struct ContextManager {
    projects: Vec<Project>,
    tasks: Vec<Task>
}

impl ContextManager {
    pub fn new() -> Self {
        ContextManager {
            projects: vec![],
            tasks: vec![]
        }
    }
}
