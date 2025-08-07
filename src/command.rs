//! Module for handling yalc commands that can be executed.
//!
//! Provides logic for parsing and executing commands.
//! Other modules may be using to execute commands.
//!

use crate::{
    config,
    constants::{DEFAULT_CONFIG_PATH, YALC_VERSION},
};

use std::{
    io::{self, ErrorKind},
    path::Path,
};

/// Enum representing different commands that can be executed
#[derive(Debug)]
pub enum Command {
    /// Help command to show available commands and descriptions
    Help,

    /// Version command to show the current program version
    Version,

    /// Config command which always has one argument
    Config(ConfigArg),

    /// Run command to execute with additional arguments
    Run(Vec<RunArg>),
}

/// Enum representing different config command arguments
#[derive(Debug)]
pub enum ConfigArg {
    /// Crates a new config file with default values
    Init,

    /// Check if the config file exists and is valid
    Check,
}

/// Enum representing different run arguments
#[derive(Debug)]
pub enum RunArg {
    /// Overwrite the config value 'dry_run' with true
    DryRun,

    /// Overwrite the config value 'missing_files_ok' with true
    MissingFilesOk,

    /// Overwrite the config value 'copy_truncate' with true
    Truncate,
}

impl Command {
    pub fn from_args(mut args: Vec<String>) -> Command {
        //First entry is called program name
        args.remove(0);

        //Execute run without any additional args
        if args.is_empty() {
            return Command::Run(vec![]);
        }

        match args[0].to_lowercase().as_str() {
            "help" => Command::Help,
            "version" | "-v" | "v" => Command::Version,
            "config" | "-c" | "c" => Self::parse_config_command(&args),
            "run" => {
                //All remaining args after run are parsed as run args
                match Self::parse_run_args(&args[1..].to_vec()) {
                    Ok(run_args) => Command::Run(run_args),
                    Err(e) => {
                        eprintln!("{}", e);
                        Command::Help
                    }
                }
            }
            _ => {
                //Execute run by default
                match Self::parse_run_args(&args) {
                    Ok(run_args) => Command::Run(run_args),
                    Err(e) => {
                        eprintln!("{}", e);
                        Command::Help
                    }
                }
            }
        }
    }

    fn parse_config_command(args: &Vec<String>) -> Command {
        //Use the check command when config is called without additional args
        if args.len() == 1 {
            Command::Config(ConfigArg::Check)
        } else if args.len() == 2 {
            //Parse the config argument command
            match args[1].to_lowercase().as_str() {
                "init" => Command::Config(ConfigArg::Init),
                "check" => Command::Config(ConfigArg::Check),
                _ => {
                    //Display help in case of invalid config arg
                    eprintln!("Invalid config argument: {}", args[1]);
                    Command::Help
                }
            }
        } else {
            //Invalid config argument length
            eprintln!(
                "Invalid amount of config arguments provided: {}",
                args.len()
            );
            Command::Help
        }
    }

    fn parse_run_args(args: &Vec<String>) -> Result<Vec<RunArg>, io::Error> {
        let mut run_args: Vec<RunArg> = Vec::with_capacity(args.capacity());

        //Convert each argument
        for arg in args.iter() {
            match arg.to_lowercase().as_str() {
                "--dry" | "-d" => {
                    run_args.push(RunArg::DryRun);
                }
                "--ignore-miss" | "-i" => {
                    run_args.push(RunArg::MissingFilesOk);
                }
                "--trunc" | "-t" => {
                    run_args.push(RunArg::Truncate);
                }
                _ => {
                    //Invalid argument
                    return Err(io::Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid run argument: '{}'", arg),
                    ));
                }
            }
        }

        Ok(run_args)
    }

    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Help => {
                println!("Available commands:");
                println!("  help       - Show this help");
                println!("  version    - Show version number of the program");
                println!("  run [ARGS] - Execute the run command with args");
                Ok(())
            }
            Command::Version => {
                println!("yalc version {}", YALC_VERSION);
                Ok(())
            }
            Command::Config(config_arg) => match &config_arg {
                ConfigArg::Init => {
                    println!("Executing: Config init");
                    config::execute_init_config_command()?;
                    Ok(())
                }
                ConfigArg::Check => {
                    println!("Executing: Config check");
                    config::execute_check_config_command()?;
                    Ok(())
                }
            },
            Command::Run(run_args) => {
                //Always load from the default config path
                let config_path = Path::new(DEFAULT_CONFIG_PATH);

                //Load the config
                match config::load_config(&config_path) {
                    Err(e) => {
                        println!("Yalc config check: [ERROR]");
                        eprintln!("Config error: {}", e);
                    }
                    Ok(raw_config) => {
                        println!("Yalc config check: [VALID]");

                        //Adjust the config based on the provided cli args
                        let config = config::adjust_runner_config(raw_config, &run_args);
                        println!("adjusted config: {:?}", config);
                    }
                }

                Ok(())
            }
        }
    }
}
