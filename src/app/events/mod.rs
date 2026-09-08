use crate::app::base::{Target, projects::Project, tasks::{ETaskState, SubTask, Task}};

pub mod input;

#[derive(Clone)]
pub enum DatabaseRequest {
    Get(Target, Option<i64>),
    Add(Target),
    UpdateState(Target, ETaskState, i64),
    Delete(Target, i64)
}

#[derive(Clone)]
pub enum Request {
    Database(DatabaseRequest),
    Exit,
    Nothing
}

#[derive(Clone)]
pub enum DatabaseResponse {
    Projects(Vec<Project>),
    Tasks(Vec<Task>),
    SubTasks(Vec<SubTask>),
    All(Vec<Project>, Vec<Task>)
}

#[derive(Clone)]
pub enum Response {
    Database(DatabaseResponse),
    Nothing
}
