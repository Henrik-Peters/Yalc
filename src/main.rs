use crate::command::Command;
use std::env;

mod command;
mod config;
mod constants;

fn main() {
    //Get arguments passed to this program
    let args: Vec<String> = env::args().collect();

    //Parse and execute command
    let command = Command::from_args(args);
    command.execute();
}
