pub mod projects;
pub mod tasks;

#[derive(Clone)]
pub enum Target {
    Project,
    Task,
    SubTask
}
