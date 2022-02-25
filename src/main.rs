mod service;
mod config;
mod protocol;

use std::sync::Arc;
use bunker::{server::{self, RouteMapBuilder}, registerable::{Route, Controller}};
use config::Container;
use service::{logger::LoggerController, hash::HashController};

use crate::protocol::{StatusError, SecondaryStatus};

fn main() {
    BunkerConfiguration(Container::init())
        .configure()
        .run()
}

struct BunkerConfiguration(Arc<Container>);

impl BunkerConfiguration {
    pub fn configure(self) -> bunker::server::Host {
        server::Builder::new()
            .read_buffer_size(1024)
            .threads(12)
            .parse_position(2)
            .debugger_level_standard()
            .configure_routes(move |rmb| { self.routes(rmb) })
            .build()
    }
    
    fn routes(&self, rmb: RouteMapBuilder) -> RouteMapBuilder {
        rmb.register(
            Box::new(LoggerController::new(Arc::clone(&self.0))), 
            Route::Path("lo".to_string())
        ).register(
            Box::new(HashController::new(Arc::clone(&self.0))), 
            Route::Path("ha".to_string())
        ).register(
            Box::new(DefaultController(Arc::clone(&self.0))),
            Route::NotFound
        )
    }
}

pub struct DefaultController(Arc<Container>);

impl Controller for DefaultController {
    fn serve(&self, _msg: String, _out_debug: std::rc::Rc<std::cell::RefCell<String>>) -> String {
        self.0.su_factory
            .create_blank()
            .set_error(StatusError::Client)
            .set_secondary(SecondaryStatus::Service)
            .build_response()
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::{StatusReport, StatusError, SecondaryStatus};

    
    #[test]
    fn response_build() {
        let actual = StatusReport::new()
            .set_error(StatusError::Client)
            .set_secondary(SecondaryStatus::Service)
            .build_response();

        let expected = "11";

        println!("Expected: {}\n\tActual: {}", expected, &actual);

        assert!(expected == &actual)
    }
}