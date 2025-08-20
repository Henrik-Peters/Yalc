//! Module for printing the help text message
//!

/// Prints a formatted help message in a man-page-like style.
pub fn print_help() {
    println!("NAME");
    println!("    yalc - A command line tool to cleanup log files");
    println!();
    println!("SYNOPSIS");
    println!("    yalc [COMMAND] [OPTIONS]");
    println!();
    println!("DESCRIPTION");
    println!(
        "    Yalc is a simple CLI tool for cleaning up log files based on a configuration file."
    );
    println!();
    println!("COMMANDS");
    println!("    help, -h, h, ?");
    println!("        Display this help message.");
    println!();
    println!("    version, -v, v");
    println!("        Display the current program version.");
    println!();
    println!("    config, -c, c [SUBCOMMAND]");
    println!(
        "        Performs actions related to the yalc configuration file. If no subcommand is"
    );
    println!("        specified, 'check' is used.");
    println!();
    println!("    run [OPTIONS]");
    println!("        Executes the log file cleanup process based on the current configuration.");
    println!("        This is the default command if no other command is provided.");
    println!();
    println!("CONFIG SUBCOMMANDS");
    println!("    init");
    println!("        Create a new default configuration file at the default config path.");
    println!();
    println!("    check");
    println!("        Check if the configuration file exists and is valid.");
    println!();
    println!("RUN OPTIONS");
    println!("    --dry, -d");
    println!("        Simulate the cleanup process without deleting or modifying any files.");
    println!();
    println!("    --ignore-miss, -i");
    println!(
        "        Do not return an error if a log file specified in the configuration is missing."
    );
    println!();
    println!("    --trunc, -t");
    println!(
        "        Truncate files instead of deleting them. This is useful for clearing files that"
    );
    println!("        are still in use by a process.");
    println!();
    println!("EXAMPLES");
    println!("    $ yalc help");
    println!("    $ yalc -d");
    println!("    $ yalc config init");
    println!("    $ yalc run --trunc --ignore-miss");
}
