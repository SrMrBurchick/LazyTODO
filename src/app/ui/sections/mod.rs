pub mod base;
pub mod spaces_section;
pub mod content_section;
pub mod inspector_section;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ESubsectionId {
    Projects,
    Tasks(Option<i64>), // Project id
    SubTasks(Option<i64>) // Task id
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ESectionId {
    Spaces,
    Content(Option<ESubsectionId>),
    Inspector
}
