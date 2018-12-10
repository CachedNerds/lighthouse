use crate::config::Config;
use crate::db;
use actix::Addr;

#[derive(Clone)]
pub struct AppState {
    pub domain: String,
    pub db: Addr<db::DbExecutor>,
}

impl AppState {
    pub fn from(config: &Config, db: &Addr<db::DbExecutor>) -> AppState {
        AppState {
            domain: config.domain.clone(),
            db: db.clone(),
        }
    }
}
