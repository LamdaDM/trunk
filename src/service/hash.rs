use std::{sync::Arc, num::ParseIntError, fmt};

use bunker::registerable;

use crate::{config::Container, protocol::{StatusSource, StatusReport, StatusError}};

const DELIMITER: char = '|';

pub struct HashController {
    container: Arc<Container>
}

impl HashController {
    pub fn new(container: Arc<Container>) -> HashController {
        HashController { container }
    }

    fn create(&self, msg: &str) -> Result<String, HashError> {
        let cfg = &self.container.cfg;

        let parsed_input: Vec<&str> = msg
            .split(DELIMITER)
            .collect();
    
        let inlen = parsed_input.len();
    
        let mem_cost: u32;
        let time_cost: u32;
        let lanes: u32;
    
        if inlen == 4 {
            mem_cost = match parsed_input[1].parse::<u32>() {
                Ok(val) => val * 1024,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };
            time_cost = match parsed_input[2].parse::<u32>() {
                Ok(val) => val,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };
            lanes = match parsed_input[3].parse::<u32>() {
                Ok(val) => val,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };
        } else if inlen == 1 {
            mem_cost = match cfg.get_val("A2_MEM").unwrap().parse::<u32>() {
                Ok(val) => val * 1024,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };
            time_cost = match cfg.get_val("A2_ITER").unwrap().parse::<u32>() {
                Ok(val) => val,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };
            lanes = match cfg.get_val("A2_LANES").unwrap().parse::<u32>() {
                Ok(val) => val,
                Err(err) => return Err(HashError::AtoiConv(err)),
            };    
        } else { 
            return Err(HashError::Parse(msg.to_string()));
        }; 
    
        Ok(argon2::hash_encoded(
            parsed_input[0].as_bytes(), 
            b"thisisasalt...acc0C)C)Aca.12.a/cadpadA{}D", 
            &argon2::Config {
                variant: argon2::Variant::Argon2i,
                version: argon2::Version::Version13,
                mem_cost,
                time_cost,
                lanes,
                thread_mode: argon2::ThreadMode::Parallel,
                secret: &[],
                ad: &[],
                hash_length: 32
            }
        ).unwrap_or(String::new()))
    }

    fn verify(&self, msg: &str) -> Result<String, HashError> {
        match msg.split_once(DELIMITER) {
            Some((original, hash)) => {
                let v = argon2::verify_encoded(hash, original.as_bytes()).unwrap();
                Ok(match v {
                    true => "t",
                    false => "f",
                }.to_string())
            },
            None => Err(HashError::Parse(msg.to_string())),
        }
    }
}

impl registerable::Controller for HashController {
    fn serve(&self, msg: String, out_debug: std::rc::Rc<std::cell::RefCell<String>>) -> String {
        let mut status_report = self.container.su_factory.create_blank();

        // input should be formatted as `[$*][...origin][|][error]`. 
        let (route, msg) = msg.split_at(1);

        status_report = match route {
            "+" => {
                match self.create(msg) {
                    Ok(hash) => status_report.set_response(hash),
                    Err(err) => {
                        out_debug.replace(err.to_string());
                        err.report(status_report)
                    },
                }
            },
            "=" => {
                match self.verify(msg) {
                    Ok(case) => status_report.set_response(case),
                    Err(err) => {
                        out_debug.replace(err.to_string());
                        err.report(status_report)
                    },
                }
            },
            _ => {
                let err = HashError::NotFound(route.to_string());
                out_debug.replace(err.to_string());
                err.report(status_report)
            }
        };

        status_report.build_response()
    }    
}

pub enum HashError {
    AtoiConv(ParseIntError),
    Parse(String),
    NotFound(String)
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashError::AtoiConv(ref err) => write!(f, "Failed to parse argument: {}", err),
            HashError::Parse(ref msg) => write!(f, "Failed to parse message: {}", msg),
            HashError::NotFound(ref route) => write!(f, "No matching route found: {}", route),
        }
    }
}

impl StatusSource for HashError {
    fn report(&self, origin: StatusReport) -> StatusReport {
        match self {
            HashError::AtoiConv(_) => origin.set_error(StatusError::Client),
            HashError::Parse(_) => origin.set_error(StatusError::Client),
            HashError::NotFound(_) => origin.set_error(StatusError::Client),
        }
    }
}