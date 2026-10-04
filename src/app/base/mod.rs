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
    // Id / Title / Description / ProjectId / ProjectTitle / Total / Completed / Progress
    Task(i64, String, String, Option<i64>, Option<String>, i64, i64, i64),
    // Id / Title / Description / ParentTaskId / ParentTaskTitle / ProjectId / ProjectTitle
    SubTask(i64, String, String, i64, String, Option<i64>, Option<String>),
}
