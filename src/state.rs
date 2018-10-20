use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub domain: String,
}

impl AppState {
    pub fn from(config: &Config) -> AppState {
        AppState {
            domain: config.domain.clone(),
        }
    }
}
