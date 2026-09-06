use tokio::sync::mpsc;

use crate::app::events::{Request, Response};

pub trait Worker {
    type WorkerRequest;

    fn handle_request(&self, request: Self::WorkerRequest) -> Result<Response, String>;
}
