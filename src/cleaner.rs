//! Module for the yalc log file cleanup execution
//!
//! Provides logic for executing cleanup tasks based on the config input.
//! Each file will be processed, even if there is an error for the other files.
//!

use std::io;

use crate::config::Config;

pub fn run_cleanup(config: &Config) -> Result<(), io::Error> {
    //Log the execution start for the cleanup
    println!(
        "Starting cleanup tasks for: {} files",
        config.file_list.len()
    );

    //Check if the file list is empty
    if config.file_list.is_empty() {
        println!("File list is empty - nothing to do");
        Ok(())
    } else {
        //Run the cleanup task for each individual file
        for (idx_task, file) in config.file_list.iter().enumerate() {
            println!("[{}] Running task for: {}", idx_task, file);

            match run_file_cleanup(idx_task, &config) {
                Ok(_) => {
                    println!("[{}] Task was successfully executed", idx_task);
                }
                Err(e) => {
                    eprintln!("[{}] Task error: {}", idx_task, e);
                }
            }

            //Separation for better log readability
            if idx_task < config.file_list.len() {
                println!("----------------");
            }
        }

        Ok(())
    }
}

fn run_file_cleanup(idx_task: usize, config: &Config) -> Result<(), io::Error> {
    Ok(())
}
