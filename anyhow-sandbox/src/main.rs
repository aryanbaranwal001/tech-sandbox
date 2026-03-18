use anyhow::{Context, Result};
use std::fmt;

#[derive(Debug)]
struct ConfigError;
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "configuration file is missing")
    }
}
impl std::error::Error for ConfigError {}

#[derive(Debug)]
struct NetworkError(u32);
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "connection failed with status: {}", self.0)
    }
}
impl std::error::Error for NetworkError {}

fn load_config() -> Result<(), ConfigError> {
    Err(ConfigError)
}

fn connect_to_server() -> Result<(), NetworkError> {
    Err(NetworkError(500))
}

fn run_app() -> Result<u32> {
    connect_to_server()?;
    load_config()?;
    // connect_to_server().context("Failed to reach API")?;
    // load_config().context("Failed to initialize system")?;

    Ok(0)
}

fn main() {
    if let Err(e) = run_app() {
        // Print the "root cause" and the "context chain"
        // eprintln!("Error: {}", e);

        eprintln!("Errors ---");
        println!("{:?}", e);
    }
}
