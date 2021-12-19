use bunker::{exception::BunkerError, registerable};

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
    let parsed_input: Vec<&str> = msg
        .split('|')
        .collect();

    if parsed_input.len() != 4 { return Err(BunkerError::BadRequest("s".to_string())) }; // TODO: Proper error message for bad formatting

    let mem_cost = match parsed_input[1].parse::<u32>() {
        Ok(val) => val * 1024,
        Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
    };
    let time_cost = match parsed_input[2].parse::<u32>() {
        Ok(val) => val,
        Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
    };
    let lanes = match parsed_input[3].parse::<u32>() {
        Ok(val) => val,
        Err(err) => return Err(BunkerError::BadRequest(err.to_string())),
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
    ).unwrap())
    
}