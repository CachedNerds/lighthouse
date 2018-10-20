use serde_json;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    // The address to listen for connections on. Defaults to 127.0.0.1
    pub bind_address: String,
    // The port to listen for connections on. Defaults to 8000
    pub port: u16,
    // The DNS name for the server. Defaults to "localhost"
    pub domain: String,
}

pub fn load_config(filepath: &Path) -> Result<Config, Box<Error>> {
    let file = File::open(filepath)?;
    let contents = serde_json::from_reader(file)?;

    Ok(contents)
}
