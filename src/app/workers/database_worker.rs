use crate::{app::{base::{Target, projects::Project, tasks::Task}, events::{DatabaseRequest, DatabaseResponse, Request, Response}, workers::base::worker::Worker}, core::database_manager::Database};

pub struct DatabaseWorker {
    database: Database
}

impl DatabaseWorker {
    pub fn new(database: Database) -> Self {
        DatabaseWorker { database: database }
    }
}

impl Worker for DatabaseWorker {
    type WorkerRequest = DatabaseRequest;

    fn handle_request(&self, request: DatabaseRequest) -> Result<Response, String> {
        match request {
            DatabaseRequest::Get(target, id) => {
                match target {
                    Target::Project => {
                        match self.database.get_projects() {
                            Ok(projects) => {
                                Ok(Response::Database(DatabaseResponse::Projects(projects)))
                            },
                            Err(e) => {
                                Err(e.to_string())
                            },
                        }
                    },
                    Target::Task => {
                        match self.database.get_tasks(id) {
                            Ok(tasks) => {
                                Ok(Response::Database(DatabaseResponse::Tasks(tasks)))
                            },
                            Err(e) => {
                                Err(e.to_string())
                            },
                        }
                    },
                    Target::SubTask => {
                        match self.database.get_sub_tasks_for_task(id.unwrap()) {
                            Ok(tasks) => {
                                Ok(Response::Database(DatabaseResponse::SubTasks(tasks)))
                            },
                            Err(e) => {
                                Err(e.to_string())
                            },
                        }
                    },
                    Target::All => {
                        let response_projects: Vec<Project>;
                        let response_tasks: Vec<Task>;
                        match self.database.get_projects() {
                            Ok(projects) => {
                                response_projects = projects;
                            },
                            Err(e) => {
                                return Err(e.to_string())
                            },
                        }

                        match self.database.get_tasks(None) {
                            Ok(tasks) => {
                                response_tasks = tasks;
                            },
                            Err(e) => {
                                return Err(e.to_string())
                            },
                        }

                        Ok(Response::Database(DatabaseResponse::All(response_projects, response_tasks)))
                    }
                    _ => {
                        Err("Unknown target".to_string())
                    },
                }

            }
            DatabaseRequest::Delete(target, id) => {
                Ok(Response::Nothing)
            }
            DatabaseRequest::UpdateState(target, id, new_state) => {
                Ok(Response::Nothing)
            }
            DatabaseRequest::Add(target) => {
                Ok(Response::Nothing)
            }
        }
    }
}

