use std::fmt;

use crate::app::{base::{
    Target, TargetInfo, TargetStatusInfo, projects::Project, tasks::{ETaskState, SubTask, Task},
}, ui::views::popup_view::PopupData};

pub mod event_listener;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DatabaseRequest {
    Get(Target, Option<i64>),
    GetInfo(Target, Option<i64>),
    Add(TargetInfo),
    UpdateState(Target, ETaskState, i64),
    Delete(Target, i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryRequest {
    GetPrevious,
    GetNext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationRequest {
    GetPrevious,
    GetNext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionRequest {
    Selected(Target, Option<i64>, Option<i64>),
    Nothing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopupRequest{
    Update(TargetInfo),
    Create(TargetInfo),
    Delete(TargetInfo),
    Confirm,
    Nothing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Database(DatabaseRequest),
    History(HistoryRequest),
    Selection(SelectionRequest),
    Popup(PopupRequest),
    Exit,
    Nothing,
}


#[derive(Clone, Debug)]
pub enum PopupResponse{
    Show(PopupData)
}

#[derive(Clone, Debug)]
pub enum DatabaseResponse {
    Projects(Vec<Project>),
    Tasks(Vec<Task>),
    Info(Option<i64>, TargetStatusInfo),
    SubTasks(Vec<SubTask>),
    All(Vec<Project>, Vec<Task>),
    Updated(Target, ETaskState, Option<i64>),
}

#[derive(Clone, Debug)]
enum HistoryResponse {
    Previous(Option<DatabaseResponse>),
    Next(Option<DatabaseResponse>),
}

#[derive(Clone, Debug)]
pub enum SelectionResponse {
    Selected(Target, Option<i64>, Option<i64>),
    Nothing,
}

#[derive(Clone, Debug)]
pub enum Response {
    Database(DatabaseResponse),
    History(HistoryResponse),
    Selection(SelectionResponse),
    Popup(PopupResponse),
    Nothing,
}

impl fmt::Display for DatabaseResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Database response:")?;
        match self {
            DatabaseResponse::Projects(projects) => {
                for project in projects {
                    writeln!(f, "{project}")?;
                }
            },
            DatabaseResponse::All(projects, tasks) => {
                for project in projects {
                    writeln!(f, "{project}")?;
                }

                for task in tasks {
                    writeln!(f, "{task}")?;
                }
            },
            DatabaseResponse::Tasks(tasks) => {
                for task in tasks {
                    writeln!(f, "{task}")?;
                }
            },
            DatabaseResponse::SubTasks(tasks) => {
                for task in tasks {
                    writeln!(f, "{task}")?;
                }
            },
            DatabaseResponse::Updated(target, state, id) => {
                writeln!(f, "{target} {state} {:?}", id)?;
            }
            _ => {
                writeln!(f, "Unknown database response")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Database(response) => {
                writeln!(f, "{response}")?;
            },
            _ => {
                writeln!(f, "Unknown response")?;
            }
        };
        Ok(())
    }
}
