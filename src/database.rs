use crate::Result;
use redis::{Client, Commands, Connection};

use std::collections::HashSet;
use std::env;

// Wraps around a database connection and provides the api to read/write sets of urls
// and read their length.
pub struct DataBaseConnection {
    connection: Connection,
}

impl DataBaseConnection {
    // Creates a new connection with the database_url environment variable.
    pub fn new() -> Result<Self> {
        let address =
            env::var("DATABASE_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1/"));
        Ok(DataBaseConnection {
            connection: Client::open(address.as_str())?.get_connection()?,
        })
    }

    // Returns a set of urls if it exists in the Redis database
    pub fn get_set(&mut self, domain_name: &str) -> Option<HashSet<String>> {
        match self.connection.exists::<&str, bool>(domain_name) {
            Ok(exists) if exists => self.connection.smembers(domain_name).ok(),
            _ => None,
        }
    }

    // Returns the length of a set of urls if it exists in the Redis database
    pub fn get_len<'a>(&mut self, domain_name: &'a str) -> Option<(&'a str, usize)> {
        match self.connection.scard(domain_name) {
            Ok(size) if size > 0 => Some((domain_name, size)),
            _ => None,
        }
    }

    // Inserts a set in the Redis database
    pub fn set(&mut self, domain_name: &str, domain_set: HashSet<String>) -> Result<()> {
        println!("insertion in the database with name: {}", domain_name);
        Ok(self.connection.sadd(domain_name, domain_set)?)
    }
}

mod tests {
    #[test]
    fn test_database_connection() {
        use super::*;
        let connection = DataBaseConnection::new().unwrap();
        assert!(connection.connection.is_open());
    }
}
