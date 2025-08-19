//! Module for the yalc log file cleanup execution
//!
//! Provides logic for executing cleanup tasks based on the config input.
//! Each file will be processed, even if there is an error for the other files.
//!

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::config::{CleanUpMode, Config};

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

            match run_file_cleanup(idx_task, &config) {
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
/// The task_idx is the 0-based index for the file in the config's file_list.
fn run_file_cleanup(task_idx: usize, config: &Config) -> Result<(), io::Error> {
    let task_nr = task_idx + 1;

    //1. Get file path from the config's file list
    let file_path_str = &config.file_list[task_idx];
    let file_path = Path::new(file_path_str);

    //2. Check for file existence and type
    if !file_path.exists() {
        if config.missing_files_ok {
            println!(
                "[{}] File not found, missing file is configured as okay",
                task_nr,
            );
            return Ok(());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", file_path.display()),
            ));
        }
    }

    //Check that the path is a file
    if !file_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path is not a file: {}", file_path.display()),
        ));
    }

    //3. Check if a cleanup is needed for the current file
    let cleanup_needed: bool = check_cleanup_conditions(task_nr, &file_path, &config)?;

    //4. If no cleanup conditions are met, we are done with this file.
    if !cleanup_needed {
        println!("[{}] No cleanup conditions met", task_nr,);
        return Ok(());
    }

    //5. Handle dry run: log action and exit without changes
    if config.dry_run {
        println!(
            "[{}] DRY RUN: Would cleanup file '{}'",
            task_nr,
            file_path.display()
        );
        return Ok(());
    }

    //6. Perform the actual file operations
    perform_file_cleanup(task_nr, &file_path, &config)?;
    Ok(())
}

/// Check if the cleanup should be performed for a given file and config
fn check_cleanup_conditions(
    task_nr: usize,
    file_path: &Path,
    config: &Config,
) -> Result<bool, io::Error> {
    //Evaluate if a cleanup is required based on the mode
    let metadata = fs::metadata(file_path)?;
    let mut cleanup_needed = false;

    //Check file size condition
    if matches!(config.mode, CleanUpMode::FileSize | CleanUpMode::All) {
        let size_limit_bytes = config.retention.file_size_mb * 1024 * 1024;

        if metadata.len() > size_limit_bytes {
            println!(
                "[{}] Condition met: File size ({} MiB) exceeds limit ({} MiB)",
                task_nr,
                metadata.len() / 1024 / 1024,
                config.retention.file_size_mb
            );
            cleanup_needed = true;
        }
    }

    //Check last write time condition, only if not already triggered
    if !cleanup_needed && matches!(config.mode, CleanUpMode::LastWrite | CleanUpMode::All) {
        let modified_time = metadata.modified()?;

        if let Ok(duration_since_write) = SystemTime::now().duration_since(modified_time) {
            let time_limit_duration =
                std::time::Duration::from_secs(config.retention.last_write_h * 3600);

            //Check if the age of the file exceeds the limit
            if duration_since_write > time_limit_duration {
                //Calculate hours for readable output
                let duration_since_write_h: u64 = duration_since_write.as_secs() / 3600;
                let time_limit_duration_h: u64 = time_limit_duration.as_secs() / 3600;

                println!(
                    "[{}] Condition met: Last write age ({} h) exceeds limit ({} h)",
                    task_nr, duration_since_write_h, time_limit_duration_h
                );
                cleanup_needed = true;
            }
        }
    }

    Ok(cleanup_needed)
}

/// Execute the cleanup or rotate operation for a file
fn perform_file_cleanup(
    task_nr: usize,
    file_path: &Path,
    config: &Config,
) -> Result<(), io::Error> {
    if config.keep_rotate == 0 {
        //If keep_rotate is 0, we just delete the file.
        println!("[{}] Removing file: keep_rotate is zero", task_nr);
        fs::remove_file(file_path)?;
    } else {
        //Rotate files by shifting them: file.1 -> file.2, file.0 -> file.1, etc.
        //This loop starts from the second to last possible rotation and moves
        //everything up one index, overwriting the oldest file in the process.
        for i in (1..config.keep_rotate).rev() {
            let source_path_str = format!("{}.{}", file_path.display(), i - 1);
            let source_path = Path::new(&source_path_str);

            if source_path.exists() {
                let dest_path_str = format!("{}.{}", file_path.display(), i);
                println!(
                    "[{}] Rotating: {} -> {}",
                    task_nr,
                    source_path.display(),
                    dest_path_str
                );
                fs::rename(source_path, &dest_path_str)?;
            }
        }

        //Handle the original file, moving it to the '.0' position
        let new_rotated_path_str = format!("{}.0", file_path.display());
        if config.copy_truncate {
            println!(
                "[{}] Copying original to '{}' and truncating",
                task_nr, new_rotated_path_str
            );
            fs::copy(file_path, &new_rotated_path_str)?;

            //Re-open the file with truncate option to clear its content while preserving the inode
            let _file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_path)?;
        } else {
            println!(
                "[{}] Renaming original to '{}'",
                task_nr, new_rotated_path_str
            );
            fs::rename(file_path, &new_rotated_path_str)?;
        }
    }

    Ok(())
}
