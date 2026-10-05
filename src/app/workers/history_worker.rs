use crate::app::{events::{DatabaseResponse, HistoryRequest, Response}, workers::base::worker::Worker};
use tracing::{info, error, warn};

pub struct HistoryWorker {
    history: Vec<Response>,
    active_response: Option<Response>
}

impl HistoryWorker {
    pub fn new() -> Self {
        HistoryWorker {
            history: vec![],
            active_response: None
        }
    }

    pub fn save_response(&mut self, response: Response) {
        match &self.active_response {
            Some(active_response) => {
                match active_response {
                    Response::Database(database_response) => {
                        match database_response {
                            DatabaseResponse::Updated(_, _, _) => {
                                return;
                            },
                            DatabaseResponse::Info(_, _) => {
                                return;
                            }
                            _ => {},
                        }
                    }
                    _ => {},
                }
                self.history.push(active_response.clone());
            },
            None => {},
        }

        self.active_response = Some(response);
    }
}

impl Worker for HistoryWorker {
    type WorkerRequest = HistoryRequest;

    fn handle_request(&mut self, request: HistoryRequest) -> Result<Response, String> {
        match request {
            HistoryRequest::GetPrevious => {
                match self.history.pop() {
                    Some(response) => {
                        self.active_response = Some(response.clone());
                        return Ok(response);
                    },
                    None => {
                        match &self.active_response {
                            Some(response) => {
                                return Ok(response.clone());
                            },
                            None => {},
                        }
                    },
                }
            },
            _ => {},
        }
        Ok(Response::Nothing)
    }
}

