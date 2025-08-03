use crate::command::Command;
use std::env;

mod cleaner;
mod command;
mod config;
mod constants;

fn main() {
    //Get arguments passed to this program
    let args: Vec<String> = env::args().collect();

    //Parse and execute command
    let command = Command::from_args(args);
    let res_command = command.execute();

    //Display the error when the command has failed
    if let Err(e) = res_command {
        eprintln!("Error: {}", e);
    }
}
