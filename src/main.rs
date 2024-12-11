//! Main module for envfetch - a lightweight cross-platform CLI tool for working with environment variables
//! 
//! This module contains the main program logic and command handlers.

mod utils;
mod cli;
mod env_ops;

use clap::Parser;
use colored::Colorize;
use std::{env, fs, process};
use dirs::home_dir;
#[cfg(windows)]
use winreg::{enums::*, RegKey};
#[cfg(not(windows))]
use which::which;

use utils::{error, run, warning};
use cli::{Cli, Commands};
use env_ops::{set_permanent_env, find_similar_vars};

/// Main entry point of the program
fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Handle different commands
    match cli.command {
        // Get command: print value of environment variable
        Commands::Get(opt) => {
            match env::var(&opt.key) {
                Ok(value) => println!("{:?}", &value),
                _ => {
                    error(format!("can't find '{}'", &opt.key).as_str(), cli.exit_on_error);
                    
                    // Show similar variable names if enabled
                    if !opt.no_similar_names {
                        if let Some(similar) = find_similar_vars(&opt.key) {
                            eprintln!("Did you mean:");
                            for name in similar {
                                eprintln!("  {}", &name);
                            }
                        }
                    }
                    process::exit(1)
                }
            }
        }
        // Print command: show all environment variables
        Commands::Print => {
            for (key, value) in env::vars() {
                println!("{} = {:?}", &key.blue(), &value);
            }
        }
        // Set command: temporarily set variable and run process
        Commands::Set(opt) => {
            unsafe { env::set_var(opt.key, opt.value) };
            run(opt.process, cli.exit_on_error);
        }
        // Gset command: permanently set variable
        Commands::Gset(opt) => {
            if let Err(err) = set_permanent_env(&opt.key, Some(&opt.value)) {
                error(err.to_string().as_str(), cli.exit_on_error);
                process::exit(1);
            }
            unsafe { env::set_var(opt.key, opt.value) };
        }
        // Delete command: temporarily delete variable and run process
        Commands::Delete(opt) => {
            if env::var(&opt.key).is_ok() {
                unsafe { env::remove_var(&opt.key) }
            } else {
                warning("variable doesn't exists");
            }
            run(opt.process, cli.exit_on_error);
        }
        // Gdelete command: permanently delete variable
        Commands::Gdelete(opt) => {
            if let Err(err) = set_permanent_env(&opt.key, None) {
                error(err.to_string().as_str(), cli.exit_on_error);
                process::exit(1);
            }
            unsafe { env::remove_var(&opt.key) };
        }
        // Load and Gload commands: load variables from file
        Commands::Load(opt) => handle_load(opt.file, Some(opt.process), cli.exit_on_error),
        Commands::Gload(opt) => handle_load(opt.file, None, cli.exit_on_error),
    }
}

/// Handle loading environment variables from a file
/// 
/// # Arguments
/// * `file` - Path to the .env file
/// * `process` - Optional process to run after loading variables
/// * `exit_on_error` - Whether to exit on error
fn handle_load(file: String, process: Option<String>, exit_on_error: bool) {
    // Try to read the file
    match fs::read_to_string(&file) {
        Ok(content) => {
            // Validate file format
            if content.contains("==") {
                error("Invalid file format: contains double equals", exit_on_error);
                process::exit(1);
            }
            
            // Parse and process variables
            match dotenv_parser::parse_dotenv(&content) {
                Ok(variables) => {
                    for (key, value) in variables {
                        // If no process specified (Gload), set variables permanently
                        if process.is_none() {
                            if let Err(err) = set_permanent_env(&key, Some(&value)) {
                                error(err.to_string().as_str(), exit_on_error);
                                process::exit(1);
                            }
                        }
                        // Set variable for current process
                        unsafe { env::set_var(key, value) };
                    }
                    // Run process if specified
                    if let Some(proc) = process {
                        run(proc, exit_on_error);
                    }
                }
                Err(err) => {
                    error(err.to_string().as_str(), exit_on_error);
                    if let Some(proc) = process {
                        run(proc, exit_on_error);
                    }
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            error(err.to_string().as_str(), exit_on_error);
            if let Some(proc) = process {
                run(proc, exit_on_error);
            }
            process::exit(1);
        }
    }
}
