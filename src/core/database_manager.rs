use sqlite::{
    Connection, State, Statement
};
use std::{
    env, fs
};

use crate::app::{projects::Project, tasks::{ETaskState, SubTask, Task}};

#[derive(Default)]
pub struct Database {
    connection: Option<Connection>
}

impl Database {
    pub fn initialize(&mut self) {
        match sqlite::open(env!("DEFAULT_DATABASE")) {
            Ok(connection) => {
                self.connection = Some(connection);
                self.initialize_tables();
                self.initialize_views();
            },
            Err(error) => {
                println!("{:?}", error);
            },
        };
    }

    fn enable_foreign_keys(&self) {
        match &self.connection {
            Some(connection) => {
                match connection.execute("PRAGMA foreign_keys = ON;") {
                    Ok(result) => {
                        println!("Foreign keys enabled successfully! {:?}", result);
                    },
                    Err(error) => {
                        println!("Failed to enable foreign_keys: {:?}", error);
                    },
                }
            },
            None => {
            },
        }

    }

    pub fn is_initialized(&self) -> bool {
        return self.connection.is_some();
    }

    pub fn load_other_source(&mut self, path: &str) {
        match sqlite::open(path) {
            Ok(connection) => {
                self.connection = Some(connection);
                self.initialize_tables();
                self.initialize_views();
            },
            Err(_) => {},
        }
    }

    fn initialize_tables(&self) {
        match &self.connection {
            Some(connection) => {
                match connection.execute(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),env!("SQL_TABLES")))) {
                    Ok(result) => {
                        println!("Database created successfully! {:?}", result);
                    },
                    Err(error) => {
                        println!("Failed to create tables: {:?}", error);
                    },
                }
            },
            None => {
            },
        }
    }

    fn initialize_views(&self) {
        match &self.connection {
            Some(connection) => {
                match connection.execute(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_VIEWS")))) {
                    Ok(result) => {
                        println!("Views created successfully! {:?}", result);
                    },
                    Err(error) => {
                        println!("Failed to create views: {:?}", error);
                    },
                }
            },
            None => {
            },
        }
    }

    pub fn create_new_task(&self, title: &str, description: &str, project_id: Option<i64>) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_CREATE_TASK")))) {
                    Ok(mut statement) => {
                        statement.bind((":title", title)).unwrap();
                        match project_id {
                            Some(id) => {
                                statement.bind((":projectId", id)).unwrap();
                            },
                            None => {},
                        }
                        statement.bind((":state", 0)).unwrap();
                        statement.bind((":description", description)).unwrap();

                        match statement.next() {
                            Ok(res) => {
                                println!("Executed successfully! {:?}", res);
                            },
                            Err(err) => {
                                println!("Failed to create new project! {:?}", err);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create task {e}")
                    },
                };
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    pub fn create_new_sub_task(&self, title: &str, description: &str, parent_task_id: i64) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_CREATE_SUB_TASK")))) {
                    Ok(mut statement) => {
                        statement.bind((":title", title)).unwrap();
                        statement.bind((":parentTaskId", parent_task_id)).unwrap();
                        statement.bind((":state", 0)).unwrap();
                        statement.bind((":description", description)).unwrap();

                        match statement.next() {
                            Ok(res) => {
                                println!("Executed successfully! {:?}", res);
                            },
                            Err(err) => {
                                println!("Failed to create new subtask! {:?}", err);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create task {e}")
                    },
                };
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    pub fn create_new_project(&self, title : &str, description: &str) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_CREATE_PROJECT")))) {
                    Ok(mut statement) => {
                        statement.bind((":title", title)).unwrap();
                        statement.bind((":description", description)).unwrap();
                        match statement.next() {
                            Ok(res) => {
                                println!("Executed successfully! {:?}", res);
                            },
                            Err(err) => {
                                println!("Failed to create new project! {:?}", err);
                            },
                        }
                    },
                    Err(_) => {},
                };
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    fn parse_task(&self, statement: &mut Statement) -> Result<Task, String> {
        let mut new_task: Task = Task::default();
        match statement.read("id") {
            Ok(value) => {
                new_task.id = value;
            },
            Err(e) => {
                eprintln!("Failed to read task ID {e}");
                return Err(e.to_string());
            },
        }

        match statement.read("projectId") {
            Ok(project_id) => {
                new_task.project_id = project_id;
            },
            Err(_) => {},
        }
        // If we have id that we have other non null values
        new_task.title = statement.read("title").unwrap();
        new_task.description = statement.read("description").unwrap();
        let value: i64 = statement.read("state").unwrap();
        match ETaskState::try_from(value) {
            Ok(state) => {
                new_task.state = state;
            },
            Err(e) => {
                eprintln!("Invalid task state! {e}");
                return Err(e);
            },
        }

        Ok(new_task)
    }


    fn parse_sub_task(&self, statement: &mut Statement) -> Result<SubTask, String> {
        let mut new_sub_task: SubTask = SubTask::default();
        match statement.read("id") {
            Ok(value) => {
                new_sub_task.id = value;
            },
            Err(e) => {
                eprintln!("Failed to read task ID {e}");
                return Err(e.to_string());
            },
        }

        // If we have id that we have other non null values
        new_sub_task.title = statement.read("title").unwrap();
        new_sub_task.description = statement.read("description").unwrap();
        new_sub_task.parent_task_id = statement.read("parentTaskId").unwrap();
        let value: i64 = statement.read("state").unwrap();
        match ETaskState::try_from(value) {
            Ok(state) => {
                new_sub_task.state = state;
            },
            Err(e) => {
                eprintln!("Invalid task state! {e}");
                return Err(e);
            },
        }

        Ok(new_sub_task)
    }

    fn parse_project(&self, statement: &mut Statement) -> Result<Project, String> {
        let mut new_project: Project = Project::default();
        match statement.read("id") {
            Ok(value) => {
                new_project.id = value;
            },
            Err(e) => {
                eprintln!("Failed to read project ID {e}");
                return Err(e.to_string());
            },
        }

        // If we have id that we have other non null values
        new_project.title = statement.read("title").unwrap();
        new_project.description = statement.read("description").unwrap();

        Ok(new_project)
    }

    fn get_sub_tasks_for_task(&self, parent_task_id: i64) -> Result<Vec<SubTask>, String> {
        let mut sub_tasks: Vec<SubTask> = vec![];
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_VIEW_GET_SUB_TASKS_BY_PARENT")))) {
                    Ok(mut statement) => {
                        statement.bind((":taskId", parent_task_id)).unwrap();
                        while let Ok(State::Row) = statement.next() {
                            match self.parse_sub_task(&mut statement) {
                                Ok(new_sub_task) => {
                                    sub_tasks.push(new_sub_task);
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                            }
                        }

                    },
                    Err(e) => {
                        eprintln!("Failed to get tasks list {e}");
                        return Err(e.to_string());
                    },
                }

                Ok(sub_tasks)
            },
            None => {
                Err("Invalid connection".to_string())
            },
        }
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>, String> {
        match &self.connection {
            Some(connection) => {
                let mut tasks: Vec<Task> = vec![];
                match connection.prepare("SELECT * FROM v_GetNonProjectTasks;") {
                    Ok(mut statement) => {
                        while let Ok(State::Row) = statement.next() {
                            match self.parse_task(&mut statement) {
                                Ok(new_task) => {
                                    tasks.push(new_task);
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to get tasks list {e}");
                        return Err(e.to_string());
                    },
                }

                for task in &mut tasks {
                    match self.get_sub_tasks_for_task(task.id) {
                        Ok(sub_tasks) => {
                            task.sub_tasks = sub_tasks;
                        },
                        Err(_) => {},
                    }
                }

                Ok(tasks)
            },
            None => {
                return Err("Invalid Connection".to_string())
            }
        }
    }

    pub fn list_projects(&self) -> Result<Vec<Project>, String> {
        match &self.connection {
            Some(connection) => {
                let mut projects: Vec<Project> = vec![];
                match connection.prepare("SELECT * FROM v_GetProjects;") {
                    Ok(mut statement) => {
                        while let Ok(State::Row) = statement.next() {
                            match self.parse_project(&mut statement) {
                                Ok(new_project) => {
                                    projects.push(new_project);
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to get projects list {e}");
                        return Err(e.to_string());
                    },
                }

                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_VIEW_GET_TASKS_BY_PROJECT")))) {
                    Ok(mut statement) => {
                        for project in &mut projects {
                            statement.bind((":projectId", project.id)).unwrap();
                            while let Ok(State::Row) = statement.next() {
                                match self.parse_task(&mut statement) {
                                    Ok(new_task) => {
                                        project.tasks.push(new_task);
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    },
                                }
                            }

                            statement.reset().unwrap();
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to get tasks list {e}");
                        return Err(e.to_string());
                    },
                }

                for project in &mut projects {
                    for task in &mut project.tasks {
                        match self.get_sub_tasks_for_task(task.id) {
                            Ok(sub_tasks) => {
                                task.sub_tasks = sub_tasks;
                            },
                            Err(_) => {},
                        }
                    }
                }

                Ok(projects)
            },
            None => {
                return Err("Invalid Connection".to_string())
            }
        }
    }

    pub fn delete_sub_task(&self, id: i64) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_SUB_TASK")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Subtask deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete subtask {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }


    pub fn delete_task(&self, id: i64) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_SUB_TASK_BY_PARENT")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Subtasks for for parent {id} deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete subtasks for {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }

                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_TASK")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Task deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete task {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }

            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    pub fn delete_project(&self, id: i64) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_SUBTASK_BY_PROJECT")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("SubTasks for project {id} was deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete subtasks for {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }

                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_TASK_BY_PROJECT")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Tasks for project {id} was deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete tasks for {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }

                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_DELETE_PROJECT")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Project {id} was deleted successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to delete project {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    pub fn update_task_state(&self, id: i64, state: ETaskState) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_UPDATE_TASK_STATE")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        statement.bind((":newState", state as i64)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Tasks {id} state was updated successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to update state for {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }

    pub fn update_sub_task_state(&self, id: i64, state: ETaskState) {
        match &self.connection {
            Some(connection) => {
                match connection.prepare(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), env!("SQL_UPDATE_SUB_TASK_STATE")))) {
                    Ok(mut statement) => {
                        statement.bind((":targetId", id)).unwrap();
                        statement.bind((":newState", state as i64)).unwrap();
                        match statement.next() {
                            Ok(_) => {
                                println!("Tasks {id} state was updated successfully");
                            },
                            Err(e) => {
                                eprintln!("Failed to update state for {id} {e}");
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create statement {e}");
                    },
                }
            },
            None => {
                eprintln!("Invalid connection");
            },
        }
    }
}
