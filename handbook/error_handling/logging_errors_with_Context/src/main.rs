use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use log::error;

fn read_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .context(format!("Unable to read configuration file from '{}'", path))
}
fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    match read_config("config.toml") {
        Ok(config) => log::info!("Configuration read: {} bytes", config.len()),
        Err(e) => {
            // Log the error using anyhow's detailed formatting
            error!("Failed to read configuration: {:?}", e);
            // Example log output:
            // ERROR [my_app] Failed to read configuration: Unable to read configuration file from 'config.toml'
            // Caused by:
            //     No such file or directory (os error 2)
        }
    }
    Ok(())
}
