//! Module for handling yalc commands that can be executed.
//!
//! Provides logic for parsing and executing commands.
//! Other modules may be using to execute commands.
//!

/// Enum representing different commands that can be executed
#[derive(Debug)]
pub enum Command {
    /// Help command to show available commands and descriptions
    Help,

    /// Version command to show the current program version
    Version,

    /// Run command to execute with additional arguments
    Run(Vec<String>),
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
            "version" | "v" => Command::Version,
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
                println!("yalc version 0.1.0");
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
