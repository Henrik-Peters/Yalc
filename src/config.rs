//! Module for the yalc cleanup config
//!
//! Provides logic for parsing and validating the config.
//! The config is used to define how a cleanup task is performed.
//!

pub mod config_commands;
pub mod toml_lexer;
pub mod toml_parser;
pub mod toml_writer;

pub use config_commands::*;

/// Represents the config for an execution of the yalc cleanup
#[derive(Debug)]
pub struct Config {
    /// If set to true operations will be logged but not executed
    pub dry_run: bool,

    /// Which mode should be evaluated to decide whether
    /// a file should be cleaned up or not
    pub mode: CleanUpMode,

    /// Number of files that are kept when a file rotation takes place.
    /// If this number is exceeded, the oldest file is deleted
    pub keep_rotate: u64,

    /// List with all file paths where log files should be processed
    pub file_list: Vec<String>,

    /// Configuration of the conditions that are checked
    /// for each file before a rotation is started
    pub retention: RetentionConfig,
}

/// Enum representing different ways to check if a file has to be cleaned up
#[derive(Debug)]
pub enum CleanUpMode {
    /// A file is cleaned up as soon as the file size
    /// from 'retention.file_size_mb' has been exceeded
    FileSize,

    /// A file is cleaned up as soon as the last write
    /// operation is older than (now-'retention.last_write_h')
    LastWrite,

    /// All cleanup modes are evaluated. A file is cleaned up
    /// if at least one condition is met (OR combination)
    All,
}

/// Represents the config values before a file cleanup should be started
#[derive(Debug)]
pub struct RetentionConfig {
    /// Size in megabytes that a file must exceed in order to be cleaned up
    file_size_mb: u64,

    /// Hours since the last write operation before a file is cleaned up
    last_write_h: u64,
}
