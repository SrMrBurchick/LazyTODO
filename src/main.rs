pub mod core;
pub mod app;

use crate::{app::app::App};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.run();
}
