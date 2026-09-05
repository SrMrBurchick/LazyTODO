pub trait DatabaseQuery {
    fn get_request(&self) -> &str;
}

pub struct AddTaskQuery;

pub struct AddProjectQuery;

pub struct AddSubTaskQuery;

impl DatabaseQuery for AddTaskQuery {
    fn get_request(&self) -> &str {
        ""
    }
}
