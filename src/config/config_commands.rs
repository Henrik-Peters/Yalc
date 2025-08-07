//! Module for the yalc config command execution
//!
//! When a config command should be executed it will be done by this module.
//! Note that the config module is also used by other non-config commands.
//! These function should help the user to configure yalc in an easy way.
//!
use std::fs::{File, metadata};
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;

use crate::command::RunArg;
use crate::config::{Config, toml_parser};
use crate::constants::{DEFAULT_CONFIG_CONTENT, DEFAULT_CONFIG_PATH};

/// This command is called via "yalc config init".
/// This will create a new default config file.
/// Will result in an error if a config file already exists.
pub fn execute_init_config_command() -> Result<(), io::Error> {
    let path = Path::new(DEFAULT_CONFIG_PATH);

    //First check if the file already exists
    if metadata(path).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Config file already exists",
        ));
    }

    //Create new config file
    create_default_config_file(path)
}

fn create_default_config_file(path: &Path) -> Result<(), io::Error> {
    println!("Creating new template config file at: {}", path.display());

    //Create new file handle
    let mut file = File::create(&path)?;

    let content = DEFAULT_CONFIG_CONTENT;
    file.write_all(content.as_bytes())?;

    //Log the successful write operation
    println!("Successfully written template config file content");
    Ok(())
}

/// This command is called via "yalc config check".
pub fn execute_check_config_command() -> Result<(), io::Error> {
    let path = Path::new(DEFAULT_CONFIG_PATH);

    //The config is validated by the load function
    match toml_parser::load_config(&path) {
        Ok(config) => {
            println!("Yalc config check: [VALID]");
            config.print_config_values();
        }
        Err(e) => {
            println!("Yalc config check: [ERROR]");
            eprintln!("Config error: {}", e);
        }
    }

    Ok(())
}

/// Load the config from a specific path
pub fn load_config(path: &Path) -> Result<Config, io::Error> {
    match toml_parser::load_config(&path) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

/// Create a new config where the cli args overwrite the config values
pub fn adjust_runner_config(config: Config, run_args: &Vec<RunArg>) -> Config {
    //Do not change the config on empty args
    if args.is_empty() {
        return config;
    }

    //Config attributes that can be overwritten
    let mut dry_run: bool = config.dry_run;
    let mut missing_files_ok: bool = config.missing_files_ok;
    let mut copy_truncate: bool = config.copy_truncate;

    for arg in args.iter() {
        match arg.to_lowercase().as_str() {
            "--dry" | "-d" => {
                dry_run = true;
            }
            "--ignore-miss" | "-i" => {
                missing_files_ok = true;
            }
            "--trunc" | "-t" => {
                copy_truncate = true;
            }
            _ => {
                //Ignore invalid args
            }
        }
    }

    let adjusted_config: Config = Config {
        dry_run,
        mode: config.mode,
        keep_rotate: config.keep_rotate,
        missing_files_ok,
        copy_truncate,
        file_list: config.file_list,
        retention: config.retention,
    };

    adjusted_config
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CleanUpMode, RetentionConfig};

    #[test]
    fn test_adjust_runner_config() {
        let raw_config: Config = Config {
            dry_run: false,
            mode: CleanUpMode::FileSize,
            keep_rotate: 3,
            missing_files_ok: false,
            copy_truncate: false,
            file_list: vec!["/var/log/my_app.log".to_string()],
            retention: RetentionConfig {
                file_size_mb: 50,
                last_write_h: 168,
            },
        };

        let args: Vec<String> = vec!["-d".to_string(), "-t".to_string()];
        let adjusted_config = adjust_runner_config(raw_config, &args);

        assert_eq!(adjusted_config.dry_run, true);
        assert_eq!(adjusted_config.missing_files_ok, false);
        assert_eq!(adjusted_config.copy_truncate, true);
    }
}
