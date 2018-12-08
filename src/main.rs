use actix_web::{http::Method, server, App};
use lighthouse::{config, routes, state::AppState};
use std::path::Path;
use std::result::Result;

fn main() {
    let loaded_config = load_config_from_default_path();
    let config_clone = loaded_config.clone();

    server::new(move || {
        App::with_state(AppState::from(&config_clone))
            .resource("/_matrix/client/versions", |r| {
                r.method(Method::GET).f(routes::version::versions)
            })
            .resource("/.well-known/matrix/client", |r| {
                r.method(Method::GET).f(routes::discovery::well_known)
            })
            .resource("/_matrix/client/r0/login", |r| {
                r.method(Method::GET)
                    .f(routes::login::supported_login_types)
            })
    })
    .bind(format!(
        "{}:{}",
        loaded_config.bind_address, loaded_config.port
    ))
    .expect(&format!("Can not bind to port {}\n", loaded_config.port))
    .run();
}

fn load_config_from_default_path() -> config::Config {
    let config_path = Path::new("./config.toml");

    match config::load_config(config_path) {
        Result::Ok(loaded_config) => loaded_config,
        Result::Err(_) => {
            println!("Failed to load config.toml, using default values.");
            config::Config {
                bind_address: String::from("127.0.0.1"),
                port: 8000,
                domain: String::from("localhost"),
            }
        }
    }
}
