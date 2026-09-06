use crate::{
    app::{commands::create_command, managers::project_manager::ProjectManager, ui::{app_widget::AppWidget, base::widget_list::WidgetListItem}}, core::{config::Config, database_manager::Database}
};

pub struct App {
    database: Database,
    config: Config,
}

impl App {
    pub fn new() -> App {
        App {
            database: Database::default(),
            config: Config::new(),
        }
    }

    pub fn run(&mut self) {
        self.initialize();

        // Test CLI
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let title = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            println!("{title} {version}");
            let mut command = create_command(args[1].as_str());
            command.construct(&args);
            match command.execute(&self.database) {
                Ok(_) => {
                    println!("Done");
                },
                Err(e) => {
                    eprintln!("Failed {e}");
                },
            }
        } else {
            let mut app_widget = AppWidget::default();
            match self.database.list_projects() {
                Ok(projects) => {
                    app_widget.list.items = projects[0].tasks.iter().cloned().map(|task| Box::new(task) as Box<dyn WidgetListItem>).collect();
                },
                Err(_) => {},
            }

            //
            match ratatui::run(|terminal| app_widget.run(terminal)) {
                _ => {},
            }
        }
    }

    // Load config data
    fn initialize(&mut self) {
        self.config.init();
        self.database.initialize();
    }
}
