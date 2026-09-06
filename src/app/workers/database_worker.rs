use crate::{app::{base::Target, events::{Request, DatabaseRequest, DatabaseResponse, Response}, workers::base::worker::Worker}, core::database_manager::Database};

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
                        match self.database.list_projects() {
                            Ok(projects) => {
                                Ok(Response::Database(DatabaseResponse::Projects(projects)))
                            },
                            Err(e) => {
                                Err(e.to_string())
                            },
                        }
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

