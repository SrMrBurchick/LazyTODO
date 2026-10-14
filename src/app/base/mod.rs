use crate::app::{base::tasks::ETaskState, ui::{views::{inspector_view::InspectorData, metric_bar_view::MetricBarData, metric_view::MetricData}}};
use std::fmt;
pub mod projects;
pub mod tasks;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Project,
    Task,
    SubTask,
    All
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetInfo {
    // Id / Title / Description
    Project(Option<i64>, String, String),
    // Id / Title / Description / ProjectId / ProjectTitle
    Task(Option<i64>, String, String, Option<i64>, Option<String>),
    // Id / Title / Description / ParentTaskId / ParentTaskTitle / ProjectId / ProjectTitle
    SubTask(Option<i64>, String, String, i64, String, Option<i64>, Option<String>),
}

// BigDICK TargetInfo
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetStatusInfo {
    // Total / Completed
    Projects(i64, i64),
    // Id / Title / Description / Total / Completed / Progress
    Project(i64, String, String, i64, i64, i64),
    // Id / Title / Description / TaskState / ProjectId / ProjectTitle / Total / Completed / Progress
    Task(i64, String, String, i64, Option<i64>, Option<String>, i64, i64, i64),
    // Id / Title / Description / TaskState / ParentTaskId / ParentTaskTitle / ProjectId / ProjectTitle
    SubTask(i64, String, String, i64, i64, String, Option<i64>, Option<String>),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Project => {
                writeln!(f, "Project")?;
            },
            Target::Task => {
                writeln!(f, "Task")?;
            },
            Target::SubTask => {
                writeln!(f, "SubTask")?;
            }
            _ => {},
        }
        Ok(())
    }
}

impl TryFrom<TargetStatusInfo> for InspectorData {
    type Error = String;

    fn try_from(value: TargetStatusInfo) -> Result<Self, Self::Error> {
        match value {
            TargetStatusInfo::Projects(total, completed) => {
                Ok(InspectorData {
                    path: vec![],
                    item_name: "PROJECTS".to_string(),
                    item_description: String::default(),
                    item_status: None,
                    metrics: vec![
                        MetricData {
                            title: "Total".to_string(),
                            value: total,
                            ..Default::default()
                        },
                        MetricData{
                            title: "Completed".to_string(),
                            value: completed,
                            ..Default::default()
                        }
                    ],
                    progress: None,
                    ..Default::default()
                })
            },
            TargetStatusInfo::Project(id, name, description, total, completed, progress) => {
                Ok(InspectorData {
                    path: vec!["Projects".to_string()],
                    item_name: name,
                    item_description: description,
                    item_status: None,
                    metrics: vec![
                        MetricData {
                            title: "Total".to_string(),
                            value: total,
                            ..Default::default()
                        },
                        MetricData{
                            title: "Completed".to_string(),
                            value: completed,
                            ..Default::default()
                        }
                    ],
                    progress: Some(MetricBarData {
                        title: "Progress".to_string(),
                        value: progress,
                        ..Default::default()
                    }),
                    identity: Some((Target::Project, id))
                })
            },
            TargetStatusInfo::Task(id, name, description, status, project_id, project_name, total, completed, progress) => {
                Ok(InspectorData {
                    path: match project_name {
                        Some(value) => {
                            vec![value]
                        },
                        None => {
                            vec![]
                        },
                    },
                    item_name: name,
                    item_description: description,
                    item_status: ETaskState::try_from(status).ok(),
                    metrics: vec![
                        MetricData {
                            title: "Total".to_string(),
                            value: total,
                            ..Default::default()
                        },
                        MetricData{
                            title: "Completed".to_string(),
                            value: completed,
                            ..Default::default()
                        }
                    ],
                    progress: Some(MetricBarData {
                        title: "Progress".to_string(),
                        value: progress,
                        ..Default::default()
                    }),
                    identity: Some((Target::Task, id))
                })
            },
            TargetStatusInfo::SubTask(id, name, description, status, parent_task_id, parent_task_name, project_id, project_name) => {
                Ok(InspectorData {
                    path: match project_name {
                        Some(value) => {
                            vec![value, parent_task_name]
                        },
                        None => {
                            vec![parent_task_name]
                        },
                    },
                    item_name: name,
                    item_description: description,
                    item_status: ETaskState::try_from(status).ok(),
                    metrics: vec![],
                    progress: None,
                    identity: Some((Target::SubTask, id))
                })

            },
            _ => Err(format!("Invalid TargetInfo"))
        }
    }

}
