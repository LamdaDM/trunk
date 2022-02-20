use std::sync::Arc;

use cfg_loader::VariableMap;
use mysql::Opts;

const CFG_FILE_PATH: &str = ".cfg";

pub struct Container {
    pub cfg: VariableMap,
    pub db: mysql::Pool
}

impl Container {
    pub fn init() -> Arc<Container> {
        let cfg = cfg_loader::load(CFG_FILE_PATH).unwrap();

        let db = mysql::Pool::new(
            Opts::from_url(
                cfg.get_val(
                    "MYSQL_CONNECTION_STRING"
                ).unwrap()
            ).unwrap()
        ).unwrap();

        Arc::new(Container{ cfg, db })
    }
}