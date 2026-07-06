use sqlite::{
    Connection
};

#[derive(Default)]
pub struct Database {
    connection: Option<Connection>
}

impl Database {
    pub fn initialize(&mut self, path: &str) {
        match sqlite::open(path) {
            Ok(connection) => {
                self.connection = Some(connection);
            },
            Err(error) => {
                println!("{:?}", error);
            },
        };
    }

    pub fn create_database(&self) {
        let querry = "
            CREATE TABLE projects (id INTEGER, name TEXT);
            CREATE TABLE tasks (id INTEGER, projectId INTEGER, name TEXT, priority INTEGER, description TEXT, comment TEXT);
            CREATE TABLE sub_tasks (id INTEGER, projectId INTEGER, parentTaskId INTEGER, name TEXT, priority INTEGER, description TEXT, comment TEXT);
        ";

        match &self.connection {
            Some(connection) => {
                match connection.execute(querry) {
                    Ok(result) => {
                        println!("Database created successfully! {:?}", result);
                    },
                    Err(error) => {
                        println!("Failed to create tables: {:?}", error);
                    },
                }
            },
            None => {
            },
        }
    }
    //
    // pub fn get_projects(&self) {
    //     let querry = "SELECT * FROM projects";
    //     match self.connection.execute(querry) {
    //         Ok(result) => {
    //             println!("Result: {:?}", result);
    //         },
    //         Err(_) => {},
    //     };
    // }
}
