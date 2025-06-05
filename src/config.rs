//! Module for the yalc cleanup config
//!
//! Provides logic for parsing and validating the config.
//! The config is used to define how a cleanup task is performed.
//!

pub mod config_commands;
pub mod toml_parser;
pub mod toml_writer;

/// Represents the config for an executing of the yalc cleanup
#[derive(Debug)]
pub struct Config {
    /// List with all file paths where log files should be cleaned
    file_list: Vec<String>,

    /// If set to true operations will be logged but not executed
    dry_run: bool,
}
