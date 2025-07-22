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

    //retention
    let file_size_mb: u64 = get_uint(&root, "keep_rotate")?;

    println!("dry_run: {:?}", dry_run);
    println!("file_size_mb: {:?}", file_size_mb);

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

fn get_value<'a>(table: &'a Table, key: &str) -> Result<&'a Value, io::Error> {
    match table.get(key) {
        Some(value) => Ok(value),
        None => Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Missing required config key: '{}'", key),
        )),
    }
}

/// Helper function to extract a boolean value
fn get_bool(table: &Table, key: &str) -> Result<bool, io::Error> {
    match get_value(&table, &key)? {
        Value::Bool(b) => Ok(*b),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Expected boolean for config key: '{}'", key),
        )),
    }
}

/// Helper function to extract an unsigned integer value
fn get_uint<T>(table: &Table, key: &str) -> Result<T, io::Error>
where
    T: Copy + TryFrom<usize>,
{
    match get_value(table, key)? {
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
