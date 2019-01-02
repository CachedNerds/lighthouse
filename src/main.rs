use actix::SyncArbiter;
use actix_web::server;
use lighthouse::app::create_app;
use lighthouse::{config, db, state::AppState};
use std::path::Path;
use std::result::Result;

fn main() {
    let loaded_config = load_config_from_default_path();
    let config_clone = loaded_config.clone();

    let sys = actix::System::new("lighthouse-home-server");

    // Start 3 db executor actors
    let manager = db::create_manager();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || db::DbExecutor(pool.clone()));

    server::new(move || create_app(AppState::from(&config_clone, &addr)))
        .bind(format!(
            "{}:{}",
            loaded_config.bind_address, loaded_config.port
        ))
        .expect(&format!("Can not bind to port {}\n", loaded_config.port))
        .start();

    println!("Running server");
    let _ = sys.run();
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
