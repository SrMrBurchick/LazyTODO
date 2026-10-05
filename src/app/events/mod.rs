use std::fmt;

use crate::app::base::{Target, TargetInfo, projects::Project, tasks::{ETaskState, SubTask, Task}};

pub mod event_listener;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DatabaseRequest {
    Get(Target, Option<i64>),
    GetInfo(Target, Option<i64>),
    Add(Target),
    UpdateState(Target, ETaskState, i64),
    Delete(Target, i64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryRequest {
    GetPrevious,
    GetNext
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationRequest {
    GetPrevious,
    GetNext
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionRequest {
    Selected(Target, Option<i64>, Option<i64>),
    Nothing
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Database(DatabaseRequest),
    History(HistoryRequest),
    Selection(SelectionRequest),
    Exit,
    Nothing
}

#[derive(Clone, Debug)]
pub enum DatabaseResponse {
    Projects(Vec<Project>),
    Tasks(Vec<Task>),
    Info(Option<i64>, TargetInfo),
    SubTasks(Vec<SubTask>),
    All(Vec<Project>, Vec<Task>),
    Updated(Target, ETaskState, Option<i64>)
}

#[derive(Clone, Debug)]
enum HistoryResponse {
    Previous(Option<DatabaseResponse>),
    Next(Option<DatabaseResponse>)
}

#[derive(Clone, Debug)]
pub enum SelectionResponse {
    Selected(Target, Option<i64>, Option<i64>),
    Nothing
}

#[derive(Clone, Debug)]
pub enum Response {
    Database(DatabaseResponse),
    History(HistoryResponse),
    Selection(SelectionResponse),
    Nothing
}

