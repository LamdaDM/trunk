use std::fmt::Display;
use std::sync::Arc;

use bunker::{exception::BunkerError, registerable};
use mysql::params;
use mysql::prelude::Queryable;
use crate::config::Container;

const DELIMITER: char = '|';

pub struct LoggerController {
    container: Arc<Container>
}

impl LoggerController {
    pub fn new(container: Arc<Container>) -> LoggerController {
        LoggerController { container }
    }

    fn persist_logs(&self, data: Model) -> Result<String, BunkerError> {
        let mut conn = self.container.db
            .get_conn()
            .unwrap();
    
        let p = conn.exec_drop(
            "INSERT INTO ext_logs (error, origin, level) VALUES (:error, :origin: level)", 
            params! {
                "error" => data.error,
                "origin" => data.origin,
                "level" => data.level.to_string()
            }  
        );
        
        match p {
            Ok(()) => Ok(String::from("Ok")),
            Err(err) => Err(BunkerError::BadRequest(String::from(err.to_string()))),
        }
    }
}

impl registerable::Controller for LoggerController {
    fn accept(&self, msg: String) -> Result<String, BunkerError> { 
        Ok(self.persist_logs(Model::new(&msg)?)?) 
    }
}

enum Level {
    Standard,
    Critical,
    Test,
    Invalid(String)
}

impl From<&str> for Level {
    fn from(level: &str) -> Level {
        match level {
            "$" => Level::Standard,
            "!" => Level::Critical,
            "?" => Level::Test,
            other => Level::Invalid(other.to_string())
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Standard => write!(f, "Standard"),
            Level::Critical => write!(f, "Critical"),
            Level::Test => write!(f, "Test"),
            Level::Invalid(given) => write!(f, "Invalid: {}", given),
        }
    }
}

struct Model {
    error: String,
    origin: String,
    level: Level
}

impl Model {
    fn new(input: &str) -> Result<Model, BunkerError> {
        let req = input.trim();

        let req: Vec<&str> = req
            .split(DELIMITER)
            .collect();

        if req.len() != 2 { return Err(BunkerError::BadRequest("Invalid request.".to_string())) }

        let (level, origin) = req[0].split_at(1);

        Ok(Model {
            error: req[1].to_string(),
            origin: origin.to_string(),
            level: Level::from(level)
        })
    }
}