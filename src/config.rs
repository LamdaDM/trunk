use std::sync::Arc;

use crate::protocol::StatusReportFactory;

const CFG_FILE_PATH: &str = ".cfg";

pub struct Container {
    pub cfg: cfg_loader::VariableMap,
    pub db: Option<mysql::Pool>,
    pub su_factory: StatusReportFactory,
}

impl Container {
    pub fn init() -> Arc<Container> {
        
        let cfg = cfg_loader::load(CFG_FILE_PATH)
            .unwrap_or_else(default_cfg);

        let db = match mysql::Opts::from_url(cfg.get_val("MYSQL_CONNECTION_STRING").unwrap()) {
            Ok(opts) => match mysql::Pool::new(opts) {
                Ok(pool) => Some(pool),
                Err(err) => {
                    println!("{}\nContinuing...", &err);
                    None
                },
            },
            Err(err) => {
                println!("{}\nContinuing...", &err);
                None
            },
        };   

        let su_factory = StatusReportFactory::new();

        Arc::new(Container{ cfg, db, su_factory })
    }
}

fn default_cfg() -> cfg_loader::VariableMap {
    let contents = String::new() 
        + "MYSQL_CONNECTION_STRING=\n"
        + "A2_MEM=256\n"
        + "A2_ITER=10\n"
        + "A2_LANES=8\n";

    cfg_loader::get_variable_map(contents)
}