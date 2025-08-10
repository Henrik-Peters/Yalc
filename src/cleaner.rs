//! Module for the yalc log file cleanup execution
//!
//! Provides logic for executing cleanup tasks based on the config input.
//! Each file will be processed, even if there is an error for the other files.
//!

use std::io;

use crate::config::Config;

/// Run all cleanup tasks for a given yalc config
pub fn run_cleanup(config: &Config) -> Result<(), io::Error> {
    //Log the execution start for the cleanup
    println!(
        "Starting cleanup tasks for: {} files",
        config.file_list.len()
    );
    println!("----------------");

    //Task status counter
    let mut tasks_executed: usize = 0;
    let mut tasks_success: usize = 0;
    let mut tasks_failure: usize = 0;

    //Check if the file list is empty
    if config.file_list.is_empty() {
        println!("File list is empty - nothing to do");
    } else {
        //Run the cleanup task for each individual file
        for (idx_task, file) in config.file_list.iter().enumerate() {
            let task_nr = idx_task + 1;
            println!("[{}] Running task for: {}", task_nr, file);

            match run_file_cleanup(task_nr, &config) {
                Ok(_) => {
                    println!("[{}] Task was successfully executed", task_nr);
                    tasks_success += 1;
                }
                Err(e) => {
                    eprintln!("[{}] Task error: {}", idx_task, e);
                    tasks_failure += 1;
                }
            }

            //Log separation for better readability
            tasks_executed += 1;
            println!("----------------");
        }
    }

    //Calculate percentage rates
    let success_rate: usize = tasks_success * 100 / tasks_executed;
    let failure_rate: usize = tasks_failure * 100 / tasks_executed;

    //Print task stats
    println!(
        "Successful tasks: {}/{} [{}%]",
        tasks_success, tasks_executed, success_rate
    );
    println!(
        "Failure tasks:    {}/{} [{}%]",
        tasks_failure, tasks_executed, failure_rate
    );

    //Log that all tasks have finished
    println!("All tasks done");
    Ok(())
}

/// Execute a single file cleanup task for a given config
fn run_file_cleanup(task_nr: usize, config: &Config) -> Result<(), io::Error> {
    Ok(())
}
