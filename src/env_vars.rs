use crate::error::{CrawlError, ErrorType};
use crate::Result;

use dotenv;

use std::env;

// Returns the right host address and port for the server.
pub fn set_env() -> Result<(String, String)> {
    // If the server is not running inside of a container, sets the environment variables from the .env file.
    if env::var("IN_CONTAINER").is_err() {
        dotenv::dotenv().map_err(|_| CrawlError::new(ErrorType::EnvError))?;
    }

    // Gets the host and port settings from environment variables, with some default values.
    let host_address = env::var("HOST_ADDRESS")?
        .parse()
        .unwrap_or_else(|_| String::from("0.0.0.0"));
    let host_port = env::var("HOST_PORT")?
        .parse()
        .unwrap_or_else(|_| String::from("3000"));
    Ok((host_address, host_port))
}