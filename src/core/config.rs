use json::JsonValue;
use std::{fs, path::Path};
use crate::core::error;

const DATABASES_FIELD: &str = "databases";

pub struct Config {
    content: JsonValue,
    // TODO: GoogleAPI creds
}

impl Config {
    pub fn new() -> Self {
        Config {
            content: JsonValue::Null
        }
    }

    pub fn init(&mut self) {
        match self.load_new(env!("DEFAULT_CONFIG")) {
            Ok(_) => {},
            Err(_) => {},
        };
    }

    pub fn load_new(&mut self, path: &str) -> Result<(), String> {
        match fs::read_to_string(path) {
            Ok(source) => {
                self.load(source.as_str());
            },
            // Skip if config is not exists
            Err(e) => {
                return Err(e.to_string());
            },
        }

        Ok(())
    }

    fn load(&mut self, source: &str) {
        match json::parse(source) {
            Ok(content) => {
                self.content = content;
            },
            Err(e) => {
                eprintln!("Failed to load config! Error: {e}");
            },
        }
    }

    pub fn save(&self) {
        match fs::write(env!("DEFAULT_CONFIG"), json::stringify(self.content.clone())) {
            Ok(_) => {
            },
            Err(e) => {
                eprintln!("Failed to save config! Error: {e}");
            },
        }
    }

    pub fn get_database_sources(&self) -> Vec<String> {
        return self.read_database_sources();
    }

    pub fn add_new_database_source(&mut self, source: String) -> Result<(), String> {
        if self.content.is_null() {
            self.create_content();
        }

        match self.content[DATABASES_FIELD].push(source) {
            Ok(_) => {},
            Err(e) => {
                return Err(e.to_string());
            }
        };

        Ok(())
    }

    fn create_content(&mut self) {
        self.content = JsonValue::new_object();
        self.content[DATABASES_FIELD] = JsonValue::new_array();
    }

    fn read_database_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = vec![];
        match self.get_value(DATABASES_FIELD) {
            Ok(value) => {
                if value.is_array() {
                    for source in value.members() {
                        sources.push(source.to_string());
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to get databases paths! Error: {e}");
            },
        }

        return sources;
    }

    fn get_value(&self, key: &str) -> Result<JsonValue, String> {
        let value: JsonValue;

        if key.is_empty() {
            return Err(format!("Unknown key {:?}", key));
        }

        if JsonValue::is_null(&self.content) {
            return Err("Config content not initialized!".to_string());
        }

        value = self.content[key].clone();

        if JsonValue::is_null(&value) {
            return Err(format!("Key {} not configured", key));
        }

        Ok(value)
    }

    fn parse(&mut self) ->error::Result<()> {
        Ok(())
    }
}
