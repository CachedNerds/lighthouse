#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate serde_json;

mod config;
mod routes;
mod state;

use actix_web::{http::Method, server, App};
use state::AppState;
use std::path::Path;
use std::result::Result;

fn main() {
    let config_json = load_config_from_default_path();
    let config_clone = config_json.clone();

    server::new(move || {
        App::with_state(AppState::from(&config_clone))
            .resource("/_matrix/client/versions", |r| {
                r.method(Method::GET).f(routes::version::versions)
            }).resource("/.well-known/matrix/client", |r| {
                r.method(Method::GET).f(routes::discovery::well_known)
            })
    }).bind(format!("{}:{}", config_json.bind_address, config_json.port))
    .expect(&format!("Can not bind to port {}\n", config_json.port))
    .run();
}

fn load_config_from_default_path() -> config::Config {
    let config_path = Path::new("./config.json");

    match config::load_config(config_path) {
        Result::Ok(loaded_config) => loaded_config,
        Result::Err(_) => {
            println!("Failed to load config.json, using default values.");
            config::Config {
                bind_address: String::from("127.0.0.1"),
                port: 8000,
                domain: String::from("localhost"),
            }
        }
    }
}
