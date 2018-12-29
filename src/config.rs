use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

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
    let mut file = File::open(filepath)?;
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;

    let config = toml::from_str(&toml_str)?;

    Ok(config)
}
