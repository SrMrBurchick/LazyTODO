pub enum CreateCommandData {
    Project,
    // project id, project title
    Task(Option<i64>, Option<String>),
    // Parent task id, parent task title, project title
    Subtask(i64, String, Option<String>)
}

pub enum CommandData {
    Create(CreateCommandData),
    Update,
    Delete,
    Confirm
}

pub trait Command {
    fn constrcut(&self);
}
