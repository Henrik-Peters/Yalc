//! Module for handling yalc commands that can be executed.
//!
//! Provides logic for parsing and executing commands.
//! Other modules may be using to execute commands.
//!

use crate::constants::YALC_VERSION;

/// Enum representing different commands that can be executed
#[derive(Debug)]
pub enum Command {
    /// Help command to show available commands and descriptions
    Help,

    /// Version command to show the current program version
    Version,

    /// Config command which always has one argument
    Config(ConfigCommandArg),

    /// Run command to execute with additional arguments
    Run(Vec<String>),
}

/// Enum representing different config command arguments
#[derive(Debug)]
pub enum ConfigCommandArg {
    /// Crates a new config file with default values
    Init,

    /// Check if the config file exists and is valid
    Check,
}

impl Command {
    pub fn from_args(mut args: Vec<String>) -> Command {
        //First entry is called program name
        args.remove(0);

        if args.is_empty() {
            return Command::Help;
        }

        match args[0].to_lowercase().as_str() {
            "help" => Command::Help,
            "version" | "-v" | "v" => Command::Version,
            "config" | "-c" | "c" => {
                //Use the check command when config is called without additional args
                if args.len() == 1 {
                    Command::Config(ConfigCommandArg::Check)
                } else if args.len() == 2 {
                    //Parse the config argument command
                    match args[1].to_lowercase().as_str() {
                        "init" => Command::Config(ConfigCommandArg::Init),
                        "check" => Command::Config(ConfigCommandArg::Check),
                        _ => Command::Help, //Display help in case of invalid config arg
                    }
                } else {
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

    pub fn execute(&self) {
        match self {
            Command::Help => {
                println!("Available commands:");
                println!("  help       - Show this help");
                println!("  version    - Show version number of the program");
                println!("  run [ARGS] - Execute the run command with args");
            }
            Command::Version => {
                println!("yalc version {}", YALC_VERSION);
            }
            Command::Config(config_arg) => {
                println!("config_arg: {:?}", config_arg);
            }
            Command::Run(args) => {
                println!("Executing 'run' with the following arguments:");
                for (i, arg) in args.iter().enumerate() {
                    println!("  Arg {}: {}", i + 1, arg);
                }
            }
        }
    }
}
