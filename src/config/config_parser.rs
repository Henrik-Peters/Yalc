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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_bool() {
        let mut root: TopLevelTable = HashMap::new();
        root.insert("dry_run".to_string(), Value::Bool(true));
        root.insert("other_key".to_string(), Value::Bool(false));

        assert_eq!(get_bool(&root, "dry_run").unwrap(), true);
        assert_eq!(get_bool(&root, "other_key").unwrap(), false);
    }

    #[test]
    fn test_get_uint() {
        let mut root: TopLevelTable = HashMap::new();
        root.insert("my_value".to_string(), Value::Integer(1234));

        let my_value: u64 = get_uint(&root, "my_value").unwrap();
        assert_eq!(my_value, 1234);

        //The value 1234 will not fit, range of u8 is [0, 255]
        let too_small: Result<u8, io::Error> = get_uint(&root, "my_value");
        assert!(too_small.is_err());
    }

    #[test]
    fn test_sub_tables() {
        let mut root: TopLevelTable = HashMap::new();
        root.insert("dry_run".to_string(), Value::Bool(false));

        let mut config_table: Table = HashMap::new();
        config_table.insert("val_a".to_string(), Value::Integer(1));
        config_table.insert("val_b".to_string(), Value::Integer(2));

        let mut servers_table: Table = HashMap::new();
        servers_table.insert("total".to_string(), Value::Integer(12));
        servers_table.insert("healthy".to_string(), Value::Integer(5));
        servers_table.insert("config".to_string(), Value::Table(config_table));

        root.insert("servers".to_string(), Value::Table(servers_table));

        //Table: root
        assert_eq!(get_bool(&root, "dry_run").unwrap(), false);

        //Table: servers
        assert_eq!(get_uint::<u64>(&root, "servers.total").unwrap(), 12);
        assert_eq!(get_uint::<u64>(&root, "servers.healthy").unwrap(), 5);

        //Table: config
        assert_eq!(get_uint::<u64>(&root, "servers.config.val_a").unwrap(), 1);
        assert_eq!(get_uint::<u64>(&root, "servers.config.val_b").unwrap(), 2);

        //Make a lookup where the final value os only a table
        let only_table: Result<u8, io::Error> = get_uint(&root, "servers.config");
        assert!(only_table.is_err());
    }
}
