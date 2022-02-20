mod service;
mod config;
mod protocol;

use std::sync::Arc;
use bunker::server;
use config::Container;
use service::{logger::LoggerController, hash::HashController};

fn main() {
    configure_builder(Container::init()).run();
}

fn configure_builder(container: Arc<Container>) -> bunker::server::Host {
    let mut builder = server::Builder::new();

    builder = builder
        .read_buffer_size(1024)
        .threads(12)
        .parse_position(2);

    builder = builder.register(
        Box::new(LoggerController::new(Arc::clone(&container))), 
        String::from("lo")
    );
        
    builder = builder.register(
        Box::new(HashController::new(Arc::clone(&container))), 
        String::from("ha")
    );
        
    builder.build()
}