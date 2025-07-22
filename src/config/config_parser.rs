//! Module for the yalc config parser
//!
//! The input of the toml_parser module is used to crate an
//! actual instance of the config. No default values are used.
//!
use std::io;
use std::io::ErrorKind;

use crate::config::{
    Config,
    toml_parser::{Table, TopLevelTable, Value},
};

/// Parse the config instance from a parsed toml top level table
pub fn parse_config(root: &TopLevelTable) -> Result<Config, io::Error> {
    //Get all attributes at the root level
    let dry_run: bool = get_bool(&root, "dry_run")?;

    println!("{:?}", dry_run);

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

/// Helper function to extract a boolean value from the config
fn get_bool(table: &Table, key: &str) -> Result<bool, io::Error> {
    match table.get(key) {
        Some(Value::Bool(b)) => Ok(*b),
        Some(_) => Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Expected boolean for config key: '{}'", key),
        )),
        None => Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Missing required config key: '{}'", key),
        )),
    }
}
