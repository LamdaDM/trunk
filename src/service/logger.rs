use std::cell::RefCell;
use std::fmt::{Display, self};
use std::rc::Rc;
use std::sync::Arc;

use bunker::registerable;
use mysql::params;
use mysql::prelude::Queryable;
use crate::config::Container;
use crate::protocol::{StatusError, SecondaryStatus, StatusSource, StatusReport};

const DELIMITER: char = '|';

pub struct LoggerController(Arc<Container>);

impl LoggerController {
    pub fn new(container: Arc<Container>) -> LoggerController {
        LoggerController(container)
    }

    fn persist_logs(&self, data: Model) -> Result<String, LoggerError> {
        let mut conn = self.0.db
            .get_conn()
            .unwrap();
    
        match conn.exec_drop(
            "INSERT INTO external_logs (error, origin, level) VALUES (:error, :origin, :level)", 
            params! {
                "error" => &data.error,
                "origin" => &data.origin,
                "level" => &data.level.to_string()
            }  
        ) {
            Ok(()) => Ok(String::new()),
            Err(err) => Err(LoggerError::Persistence(data, err.to_string())),
        }
    }
}

impl registerable::Controller for LoggerController {
    fn serve(&self, msg: String, out_debug: Rc<RefCell<String>>) -> String {
        let mut status_report = self.0.su_factory.create_blank();

        status_report = match Model::new(&msg) {
            Ok(data) => {
                match self.persist_logs(data) {
                    Ok(ok) => {
                        status_report.set_response(ok)
                    },
                    Err(err) => {
                        out_debug.replace(err.to_string());
                        err.report(status_report)
                    },
                }
                
            },
            Err(err) => {
                out_debug.replace(err.to_string());
                err.report(status_report)
            },
        };
        
        status_report.build_response()
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

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n\t{}\n\t{}\n\t{}\n}}", self.error, self.origin, self.level)
    }
}

impl Model {
    fn new(input: &str) -> Result<Model, LoggerError> {
        let trimmed_input = input.trim();

        let req: Vec<&str> = trimmed_input
            .split(DELIMITER)
            .collect();

        if req.len() != 2 { return Err(LoggerError::Binding(trimmed_input.to_string(), req.len())) }

        let (level, origin) = req[0].split_at(1);

        Ok(Model {
            error: req[1].to_string(),
            origin: origin.to_string(),
            level: Level::from(level)
        })
    }
}


enum LoggerError {
    Persistence(Model, String),
    Binding(String, usize)
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::Persistence(ref model, ref db_msg) => write!(f, 
                "Failed to persist model! Given model: {}\n\tMessage from database: {}", 
                model,
                db_msg
            ),
            LoggerError::Binding(ref msg, ref len) => write!(f, 
                "Failed to bind request paramers to model!\n\tLen: {}\n\tRequest: {}",
                len,
                msg
            ),
        }
    }
}

impl StatusSource for LoggerError {
    fn report(&self, origin: StatusReport) -> StatusReport {
        match self {
            LoggerError::Persistence(_, _) =>  origin.set_error(StatusError::Server).set_secondary(SecondaryStatus::Dependency),
            LoggerError::Binding(_, _) =>   origin.set_error(StatusError::Client).set_secondary(SecondaryStatus::None),
        }
    }
}