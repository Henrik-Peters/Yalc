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

    let file_size_mb: u64 = get_uint(&root, "retention.file_size_mb")?;

    println!("dry_run: {:?}", dry_run);
    println!("file_size_mb: {:?}", file_size_mb);

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

/// Get a value from the top level table. Use '.' to separate between sub tables
fn get_value<'a>(root: &'a TopLevelTable, key: &str) -> Result<&'a Value, io::Error> {
    //Split the key by dot to access sub tables
    let keys: Vec<&str> = key.split('.').collect();
    let mut current_table: &Table = root;

    for (i, current_key) in keys.iter().enumerate() {
        match current_table.get(*current_key) {
            Some(Value::Table(inner_table)) if i < keys.len() - 1 => {
                //We have a table and should continue processing the next key part
                current_table = inner_table;
            }
            Some(value) if i == keys.len() - 1 => {
                //We are at the last key part
                return Ok(value);
            }
            _ => {
                //Key lookup failed or value is not a table
                return Err(io::Error::new(
                    ErrorKind::NotFound,
                    format!("Missing or invalid config key: '{}'", key),
                ));
            }
        }
    }

    Err(io::Error::new(
        ErrorKind::NotFound,
        format!("Missing required config key: '{}'", key),
    ))
}

/// Helper function to extract a boolean value
fn get_bool(root: &TopLevelTable, key: &str) -> Result<bool, io::Error> {
    match get_value(&root, &key)? {
        Value::Bool(b) => Ok(*b),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Expected boolean for config key: '{}'", key),
        )),
    }
}

/// Helper function to extract an unsigned integer value
fn get_uint<T>(root: &TopLevelTable, key: &str) -> Result<T, io::Error>
where
    T: Copy + TryFrom<usize>,
{
    match get_value(root, key)? {
        Value::Integer(i) => {
            if *i >= 0 {
                let value = *i as usize;

                //Try to perform conversion to the final type
                T::try_from(value).map_err(|_| {
                    io::Error::new(
                        ErrorKind::InvalidData,
                        format!("Value for '{}' exceeds the maximum allowed value", key),
                    )
                })
            } else {
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Negative value is not allowed for config key: '{}'", key),
                ))
            }
        }
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Expected unsigned integer for config key: '{}'", key),
        )),
    }
}
