use std::{error, fmt};

#[derive(Debug, Default)]
pub struct Error {
    pub message: Option<String>,
}

pub type Result<T> = std::result::Result<T, Error>;
