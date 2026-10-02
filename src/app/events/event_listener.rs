use tokio::sync::mpsc;

use crate::app::events::{Request, Response};

pub trait EventsListener {
    type WorkerResponse;

    fn handle_response(&mut self, response: &Self::WorkerResponse) -> Result<Request, String>;
}
