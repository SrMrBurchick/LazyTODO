pub mod core;
pub mod app;

use crate::{app::app::App};

use tracing::{info, Level};
use tracing_subscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_appender::rolling;

#[tokio::main]
async fn main() {
    let log_file = rolling::never("./logs", "lazytodo.log");
    let subscriber = tracing_subscriber::fmt()
        .with_writer(log_file)
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .init();

    let mut app = App::new();
    app.run().await;
}
