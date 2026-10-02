pub mod projects;
pub mod tasks;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Project,
    Task,
    SubTask,
    All
}
