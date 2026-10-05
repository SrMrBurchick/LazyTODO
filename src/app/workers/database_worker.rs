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

    fn handle_request(&mut self, request: DatabaseRequest) -> Result<Response, String> {
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
            DatabaseRequest::UpdateState(target, new_state, id) => {
                match target {
                    Target::Task => {
                        self.database.update_task_state(id, new_state.clone());
                        return Ok(Response::Database(DatabaseResponse::Updated(target, new_state, Some(id))));
                    }
                    Target::SubTask => {
                        self.database.update_sub_task_state(id, new_state.clone());
                        return Ok(Response::Database(DatabaseResponse::Updated(target, new_state, Some(id))));
                    }
                    _ => {},
                }
                Ok(Response::Nothing)
            }
            DatabaseRequest::Add(target) => {
                Ok(Response::Nothing)
            },
            DatabaseRequest::GetInfo(target, id) => {
                match target {
                    Target::Project => {
                        match id {
                            Some(project_id) => {
                                match self.database.get_project_info(project_id) {
                                    Some(info) => {
                                        return Ok(Response::Database(DatabaseResponse::Info(Some(project_id), info)));
                                    },
                                    None => {},
                                }
                            },
                            None => {
                                match self.database.get_projects_info() {
                                    Some(info) => {
                                        return Ok(Response::Database(DatabaseResponse::Info(None, info)));
                                    },
                                    None => {},
                                }
                            },
                        }

                    },
                    Target::Task => {
                        match id {
                            Some(task_id) => {
                                match self.database.get_task_info(task_id) {
                                    Some(info) => {
                                        return Ok(Response::Database(DatabaseResponse::Info(Some(task_id), info)));
                                    },
                                    None => {},
                                }
                            },
                            None => {},
                        }
                    }
                    Target::SubTask => {
                        match id {
                            Some(subtask_id) => {
                                match self.database.get_sub_task_info(subtask_id) {
                                    Some(info) => {
                                        return Ok(Response::Database(DatabaseResponse::Info(Some(subtask_id), info)));
                                    },
                                    None => {},
                                }
                            },
                            None => {},
                        }
                    }
                    _ => {},
                }

                Ok(Response::Nothing)
            }
            _ => {
                Ok(Response::Nothing)
            }
        }
    }
}

