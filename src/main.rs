use std::net::SocketAddr;
use std::process;
use std::str::FromStr;

type Result<T> = std::result::Result<T, error::CrawlError>;

pub mod crawler;
pub mod database;
pub mod domain;
pub mod env_vars;
pub mod error;
pub mod json;
pub mod parsing;
pub mod server;


// Launches the server.
fn main() {
    match env_vars::set_env() {
        Ok((host_address, port)) => {
            let binding_address = host_address + ":" + &port;
            let addr = SocketAddr::from_str(binding_address.as_str()).unwrap();
            server::start_server(addr);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
