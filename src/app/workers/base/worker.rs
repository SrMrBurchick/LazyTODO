use tokio::sync::mpsc;

use crate::app::events::{Request, Response};

pub trait Worker {
    fn handle_request(&self, request: Request) -> Result<Response, String>;
}
