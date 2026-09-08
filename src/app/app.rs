use std::clone;

use crate::{
    app::{base::Target, commands::create_command, events::{DatabaseRequest, Request, Response}, managers::project_manager::ProjectManager, ui::{app_widget::AppWidget, base::widget_list::WidgetListItem}, workers::{base::worker::Worker, database_worker::DatabaseWorker}}, core::{config::Config, database_manager::Database}
};

use tokio::sync::mpsc;

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::new(),
        }
    }

    pub async fn run(&mut self) {
        self.initialize();

        // Test CLI
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let title = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            println!("{title} {version}");
            let mut database = Database::default();
            database.initialize();
            let mut command = create_command(args[1].as_str());
            command.construct(&args);
            match command.execute(&mut database) {
                Ok(_) => {
                    println!("Done");
                },
                Err(e) => {
                    eprintln!("Failed {e}");
                },
            }
        } else {
            let (request_tx, request_rx) = mpsc::channel::<Request>(32);
            let (response_tx, response_rx) = mpsc::channel::<Response>(32);

            let mut app_widget = AppWidget::new(request_tx, response_rx);
            let mut terminal = ratatui::init();

            tokio::spawn(async move {
                App::setup_workers(response_tx, request_rx).await;
            });


            app_widget.initialize();
            app_widget.run(&mut terminal).await;

            ratatui::restore();
        }
    }

    async fn setup_workers(tx: mpsc::Sender<Response>, mut rx: mpsc::Receiver<Request>) {
        let mut database = Database::default();
        database.initialize();
        let database_worker = DatabaseWorker::new(database);

        while let Some(request) = rx.recv().await {
            match request {
                Request::Database(db_request) => {
                    match database_worker.handle_request(db_request) {
                        Ok(response) => {
                            tx.send(response).await;
                        },
                        Err(_) => {},
                    }
                }
                Request::Exit => {
                    return;
                }
                _ => {

                }
            }
        }
    }

    // Load config data
    fn initialize(&mut self) {
        self.config.init();
    }
}
