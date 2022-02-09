use bunker::{exception::BunkerError, registerable};

use crate::dependency::VARIABLE_MAP;

const DELIMITER: char = '|';

pub struct HashController;

impl registerable::Controller for HashController {
    fn accept(&self, msg: String) -> Result<String, BunkerError> {
        serve(&msg)
    }
}

/// Takes buffer from stream and separates into strings,
/// input should be formatted as: `[$*][...origin][|][error]`.
/// Throws error from utf8-to-string conversion, string-to-u32 conversion or wrong formatting.
fn serve(msg: &str) -> Result<String, BunkerError> {
    let (route, msg) = msg.split_at(1);

    match route {
        "+" => create(msg),
        "=" => verify(msg),
        _ => Err(BunkerError::BadRequest(format!("Invalid route: {}", route)))
    }
}

fn verify(msg: &str) -> Result<String, BunkerError> {
    match msg.split_once(DELIMITER) {
        Some((original, hash)) => {
            let v = argon2::verify_encoded(hash, original.as_bytes()).unwrap();
            Ok(match v {
                true => "t",
                false => "f",
            }.to_string())
        },
        None => Err(BunkerError::BadRequest(format!("Invalid request: {}", msg))),
    }
}

fn create(msg: &str) -> Result<String, BunkerError> {
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
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };
        time_cost = match parsed_input[2].parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };
        lanes = match parsed_input[3].parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };
    } else if inlen == 1 {
        mem_cost = match VARIABLE_MAP.get_val("A2_MEM").unwrap().parse::<u32>() {
            Ok(val) => val * 1024,
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };
        time_cost = match VARIABLE_MAP.get_val("A2_ITER").unwrap().parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };
        lanes = match VARIABLE_MAP.get_val("A2_LANES").unwrap().parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
        };    
    } else { 
        return Err(BunkerError::BadRequest(
            format!("Invalid parameter length: {}", parsed_input.len())
        )) 
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