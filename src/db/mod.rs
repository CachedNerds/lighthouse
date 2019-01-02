use actix_web::actix::{Actor, SyncContext};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub mod handlers;
pub mod messages;
pub mod models;

// This is the alias for our connection type
// we can easily swap this for PgConnection or whatever db in the future
pub type ConnectionType = SqliteConnection;

pub struct DbExecutor(pub Pool<ConnectionManager<ConnectionType>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn create_manager() -> ConnectionManager<ConnectionType> {
    dotenv().ok();
    let mut manager = None;
    for (key, value) in env::vars() {
        if key == "DATABASE_URL".to_string() {
            manager = Some(ConnectionManager::<ConnectionType>::new(value.to_string()))
        }
    }
    manager.unwrap_or(ConnectionManager::<ConnectionType>::new(
        "app.db".to_string(),
    ))
}

pub fn create_manager_with_connection_string(connection_string: String) -> ConnectionManager<ConnectionType> {
    ConnectionManager::<ConnectionType>::new(connection_string)
}
