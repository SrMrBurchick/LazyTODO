use crate::{app::base::tasks::ETaskState, core::database_manager::Database};

pub enum ECommands {
    Add,
    Update,
    Delete,
    Complete,
    List,
    None
}

// commands strings
// ./lazytodo add task test "test description"
const ADD: &str = "add";

// ./lazytodo update task test "test description" ???
const UPDATE: &str = "update";

// ./lazytodo complete task test
const COMPLETE: &str = "complete";

// ./lazytodo delete task test
// ./lazytodo delete all
const DELETE: &str = "delete";

// ./lazytodo list
const LIST: &str = "list";


// targets strings
const TASK: &str = "task";
const PROJECT_TASK: &str = "project_task";
const SUBTASK: &str = "subtask";
const PROJECT: &str = "project";
const ALL: &str = "all";

pub trait CliCommand {
    fn construct(&mut self, args: &[String]);
    fn execute(&self, database: &Database) -> Result<(), String>;
}

pub trait Command {
    fn new(command: ECommands) -> Self;
}

#[derive(Default)]
pub struct AddCommand {
    pub target: String,
    pub name: String,
    pub description: String,
    pub args: Vec<String>
}

#[derive(Default)]
pub struct CompleteCommand {
    pub target: String,
    pub id: i64,
    pub state: i64
}

#[derive(Default)]
pub struct UpdateCommand {
    pub target: String,
    pub id: i64,
    pub title: String,
    pub description: String
}

#[derive(Default)]
pub struct DeleteCommand {
    pub target: String,
    pub id: i64,
}

#[derive(Default)]
pub struct ListCommand {
    pub target: String
}

impl CliCommand for AddCommand {
    fn construct(&mut self, args: &[String]) {
        self.target = args[2].clone();
        self.name = args[3].clone();
        self.description = args[4].clone();
        self.args = args.to_vec();
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        match self.target.as_str() {
            TASK => {
                database.create_new_task(&self.name, &self.description, None);
            }
            PROJECT_TASK => {
                database.create_new_task(&self.name, &self.description, Some(self.args[5].parse::<i64>().unwrap()));
            }
            PROJECT => {
                database.create_new_project(&self.name, &self.description);
            }
            SUBTASK => {
                database.create_new_sub_task(&self.name, &self.description, self.args[5].parse::<i64>().unwrap());
            }
            _ => {
                return Err("Invalid target".to_string());
            }
        }
        Ok(())
    }
}

impl CliCommand for UpdateCommand {
    fn construct(&mut self, args: &[String]) {
        self.target = args[2].clone();
        self.id = args[3].parse::<i64>().unwrap();
        self.title = args[4].clone();
        self.description = args[5].clone();
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        match self.target.as_str() {
            TASK => {
                database.update_task(self.id, &self.title, &self.description);
            }
            PROJECT_TASK => {
                database.update_task(self.id, &self.title, &self.description);
            }
            PROJECT => {
            }
            SUBTASK => {
                database.update_sub_task(self.id, &self.title, &self.description);
            }
            _ => {
                return Err("Invalid target".to_string());
            }
        }
        Ok(())
    }
}


impl CliCommand for CompleteCommand {
    fn construct(&mut self, args: &[String]) {
        self.target = args[2].clone();
        self.id = args[3].parse::<i64>().unwrap();
        self.state = args[4].parse::<i64>().unwrap();
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        match self.target.as_str() {
            TASK => {
                database.update_task_state(self.id, ETaskState::try_from(self.state).unwrap());
            }
            SUBTASK => {
                database.update_sub_task_state(self.id, ETaskState::try_from(self.state).unwrap());
            }
            _ => {
                return Err("Invalid target".to_string());
            }
        }

        Ok(())
    }
}

impl CliCommand for DeleteCommand {
    fn construct(&mut self, args: &[String]) {
        self.target = args[2].clone();
        self.id = args[3].parse::<i64>().unwrap();
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        match self.target.as_str() {
            TASK => {
                database.delete_task(self.id);
            }
            PROJECT_TASK => {
            }
            PROJECT => {
                database.delete_project(self.id);
            }
            SUBTASK => {
                database.delete_sub_task(self.id);
            }
            _ => {
                return Err("Invalid target".to_string());
            }
        }
        Ok(())
    }
}

impl CliCommand for ListCommand {
    fn construct(&mut self, args: &[String]) {
        self.target = args[2].clone();
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        match self.target.as_str() {
            TASK => {
                match database.list_tasks() {
                    Ok(tasks) => {
                        for task in tasks {
                            println!("{task}");
                        }
                    },
                    Err(_) => {},
                }
            }
            PROJECT => {
                match database.list_projects() {
                    Ok(projects) => {
                        for project in projects {
                            println!("{project}");
                        }
                    },
                    Err(_) => {},
                }
            }
            _ => {
                return Err("Invalid target".to_string());
            }
        }

        Ok(())
    }
}

pub struct UnknownCommand;
impl CliCommand for UnknownCommand {
    fn construct(&mut self, _args: &[String]) {
        // TODO
    }

    fn execute(&self, database: &Database) -> Result<(), String> {
        Err("UnknownCommand".to_string())
    }
}

pub fn create_command(command: &str) -> Box<dyn CliCommand> {
    match command {
        ADD => {
            Box::new(AddCommand::default())
        },
        DELETE => {
            Box::new(DeleteCommand::default())
        },
        COMPLETE => {
            Box::new(CompleteCommand::default())
        }
        LIST => {
            Box::new(ListCommand::default())
        }
        UPDATE => {
            Box::new(UpdateCommand::default())
        }
        _ => {
            Box::new(UnknownCommand)
        }
    }
}
