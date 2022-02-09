use std::sync::RwLock;

use cfg_loader::VariableMap;
use mysql::Opts;

const CFG_FILE_PATH: &str = ".cfg";

lazy_static! {
    pub static ref DATABASE_MYSQL: RwLock<DbMySQL> = RwLock::new(DbMySQL::load());
    pub static ref VARIABLE_MAP: VariableMap = cfg_loader::load(CFG_FILE_PATH).unwrap();
}

pub struct DbMySQL(mysql::Pool);

impl DbMySQL {
    fn load() -> DbMySQL {
        DbMySQL(
            mysql::Pool::new(
                Opts::from_url(
                    VARIABLE_MAP.get_val(
                        "MYSQL_CONNECTION_STRING"
                    ).unwrap()
                ).unwrap()
            ).unwrap())
    }
    pub fn get_conn(&self) -> mysql::PooledConn {
        self.0.get_conn().unwrap()
    }
}