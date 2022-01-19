mod service;
mod dependency;

#[macro_use]
extern crate lazy_static;

use bunker::server;
use service::{logger::LoggerController, hash::HashController};

fn main() {
    server::Builder::new()
        .read_buffer_size(1024)
        .threads(12)
        .parse_position(2)
        .register(Box::new(LoggerController), String::from("lo"))
        .register(Box::new(HashController), String::from("ha"))
        .build()
        .run();
}