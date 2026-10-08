use crate::app::{base::tasks::ETaskState, ui::{views::{inspector_view::InspectorData, metric_bar_view::MetricBarData, metric_view::MetricData}}};

pub mod projects;
pub mod tasks;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Project,
    Task,
    SubTask,
    All
}

// BigDICK TargetInfo
#[derive(Clone, Debug)]
pub enum TargetInfo {
    // Total / Completed
    Projects(i64, i64),
    // Id / Title / Description / Total / Completed / Progress
    Project(i64, String, String, i64, i64, i64),
    // Id / Title / Description / TaskState / ProjectId / ProjectTitle / Total / Completed / Progress
    Task(i64, String, String, i64, Option<i64>, Option<String>, i64, i64, i64),
    // Id / Title / Description / TaskState / ParentTaskId / ParentTaskTitle / ProjectId / ProjectTitle
    SubTask(i64, String, String, i64, i64, String, Option<i64>, Option<String>),
}

impl TryFrom<TargetInfo> for InspectorData {
    type Error = String;

    fn try_from(value: TargetInfo) -> Result<Self, Self::Error> {
        match value {
            TargetInfo::Projects(total, completed) => {
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
            TargetInfo::Project(id, name, description, total, completed, progress) => {
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
            TargetInfo::Task(id, name, description, status, project_id, project_name, total, completed, progress) => {
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
            TargetInfo::SubTask(id, name, description, status, parent_task_id, parent_task_name, project_id, project_name) => {
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
