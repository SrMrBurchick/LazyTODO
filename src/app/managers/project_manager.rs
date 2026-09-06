use crate::app::base::projects::Project;

pub struct ProjectManager {
    pub projects: Vec<Project>
}

impl ProjectManager {
    pub fn new() -> Self {
        ProjectManager {
            projects: vec![]
        }
    }
}
