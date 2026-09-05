use crate::{
    app::{commands::create_command, projects::Project, tasks::Task}, core::{config::Config, database_manager::Database}
};

pub struct App {
    database: Database,
    projects: Vec<Project>,
    tasks: Vec<Task>,
    config: Config
}

impl App {
    pub fn new() -> App {
        App {
            database: Database::default(),
            projects: vec![],
            tasks: vec![],
            config: Config::new()
        }
    }

    pub fn run(&mut self) {
        println!("LazyTODO v.0.0.0");
        self.initialize();

        // Test CLI
        let args: Vec<String> = std::env::args().collect();
        println!("{:?}", args);
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
    }

    // Load config data
    fn initialize(&mut self) {
        self.config.init();
        self.database.initialize();
    }
}
