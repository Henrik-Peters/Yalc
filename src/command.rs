//! Module for handling yalc commands that can be executed.
//!
//! Provides logic for parsing and executing commands.
//! Other modules may be using to execute commands.
//!

use crate::{
    config,
    constants::{DEFAULT_CONFIG_PATH, YALC_VERSION},
};

use std::path::Path;

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
    Run(Vec<String>),
}

/// Enum representing different config command arguments
#[derive(Debug)]
pub enum ConfigArg {
    /// Crates a new config file with default values
    Init,

    /// Check if the config file exists and is valid
    Check,
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
            "config" | "-c" | "c" => {
                //Use the check command when config is called without additional args
                if args.len() == 1 {
                    Command::Config(ConfigArg::Check)
                } else if args.len() == 2 {
                    //Parse the config argument command
                    match args[1].to_lowercase().as_str() {
                        "init" => Command::Config(ConfigArg::Init),
                        "check" => Command::Config(ConfigArg::Check),
                        _ => Command::Help, //Display help in case of invalid config arg
                    }
                } else {
                    //Invalid config argument length
                    Command::Help
                }
            }
            "run" => {
                let run_args = args[1..].to_vec();
                Command::Run(run_args)
            }
            _ => Command::Help,
        }
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
            Command::Run(args) => {
                println!("Executing 'run' with the following arguments:");
                for (i, arg) in args.iter().enumerate() {
                    println!("  Arg {}: {}", i + 1, arg);
                }

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
                        let config = config::adjust_runner_config(raw_config, &args);
                    }
                }

                Ok(())
            }
        }
    }
}
