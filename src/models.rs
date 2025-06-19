use std::error::Error;
use std::fmt::Display;

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    author,
    version,
    after_help = "Get more info at project's repo: https://github.com/ankddev/envfetch",
    after_long_help = "Get more info at project's GitHub repo available at https://github.com/ankddev/envfetch",
    arg_required_else_help = true,
    name = "envfetch"
)]
#[command(
    about = "envfetch - lightweight tool for working with environment variables",
    long_about = "envfetch is a lightweight cross-platform CLI tool for working with environment variables"
)]
pub struct Cli {
    /// Tool commands
    #[command(subcommand)]
    pub command: Commands,
}

/// All tool's commands
#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Open envfetch in interactive mode with TUI.
    Interactive,
    /// Print value of environment variable.
    Get(GetArgs),
    /// Set environment variable and optionally run given process.
    Set(SetArgs),
    /// Add value to the end of environment variable and optionally run given process.
    Add(AddArgs),
    /// Delete environment variable and optionally run given process.
    Delete(DeleteArgs),
    /// Load environment variables from dotenv file and optionally run given process.
    Load(LoadArgs),
    /// Print all environment variables.
    Print(PrintArgs),
    /// Initialize config file.
    InitConfig,
    /// Export variable to .env file
    Export(ExportArgs),
}

/// Args for print command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct PrintArgs {
    /// Set custom format, by default {name} = "{value}" is used.
    #[arg(long, short)]
    pub format: Option<String>,
}

/// Args for get command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct GetArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Disable showing similar variables' names if variable not found
    #[arg(long, short = 's', default_value = "false")]
    pub no_similar_names: bool,
}

/// Args for load command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct LoadArgs {
    /// Globally set variable
    #[arg(required = false, long, short)]
    pub global: bool,
    /// Process to start, not required if --global flag is set
    #[arg(
        last = true,
        required_unless_present = "global",
        allow_hyphen_values = true,
        num_args = 1..
    )]
    pub process: Vec<String>,
    /// Relative or absolute path to file to read variables from.
    /// Note that it must be in .env format
    #[arg(long, short, default_value = ".env")]
    pub file: String,
}

/// Args for set command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct SetArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Value for environment variable
    #[arg(required = true)]
    pub value: String,
    /// Globally set variable
    #[arg(required = false, long, short)]
    pub global: bool,
    /// Process to start, not required if --global flag is set
    // #[arg(
    //     last = true,
    //     required_unless_present = "global",
    //     num_args = 1..,
    //     value_parser = |arg: &_| {
    //         Ok::<String, String>(
    //             std::env::args_os() // Get all CLI arguments
    //                 .skip_while(|a| a != "--") // Skip until "--"
    //                 .skip(1) // Skip the "--" itself
    //                 .map(|a| a.to_string_lossy().into_owned())
    //                 .collect::<Vec<_>>()
    //                 .join(" ")
    //         )
    //     }
    // )]
    #[arg(
        last = true,
        required_unless_present = "global",
        allow_hyphen_values = true,
        num_args = 1..
    )]
    pub process: Vec<String>,
}

/// Args for add command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct AddArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Value for add to the end of environment variable
    #[arg(required = true)]
    pub value: String,
    /// Globally set variable
    #[arg(required = false, long, short)]
    pub global: bool,
    /// Process to start, not required if --global flag is set
    #[arg(
        last = true,
        required_unless_present = "global",
        allow_hyphen_values = true,
        num_args = 1..
    )]
    pub process: Vec<String>,
}

/// Args for delete command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct DeleteArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Globally set variable
    #[arg(required = false, long, short)]
    pub global: bool,
    /// Process to start, not required if --global flag is set
    #[arg(
        last = true,
        required_unless_present = "global",
        allow_hyphen_values = true,
        num_args = 1..
    )]
    pub process: Vec<String>,
}

/// Args for export command
#[derive(Args, Debug, PartialEq, Eq)]
pub struct ExportArgs {
    /// File name to be exported as
    #[arg(required = true)]
    pub file_name: String,
    /// Environment variable(s) name
    #[arg(
        required = true,
        last = true,
        num_args = 1..
    )]
    pub keys: Vec<String>,
}

#[derive(Debug)]
pub enum ErrorKind {
    StartingProcessError,
    CannotSetVariableGlobally(String),
    CannotDeleteVariableGlobally(String),
    ParsingError(String),
    FileError(String),
    CannotFindVariable(String, bool),
    NameValidationError(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigParsingError {
    FSError(String),
    ParsingError(String),
    FileDoesntExists,
}

impl Display for ConfigParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigParsingError::FSError(err) => write!(f, "Error while reading file: {}", err),
            ConfigParsingError::ParsingError(err) => write!(f, "Error while parsing file: {}", err),
            ConfigParsingError::FileDoesntExists => write!(f, "Config file doesn't exists"),
        }
    }
}

impl Error for ConfigParsingError {}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::StartingProcessError => write!(f, "Can't start process"),
            ErrorKind::CannotSetVariableGlobally(err) => {
                write!(
                    f,
                    "Can't set variable globally (try running with sudo or administrative privileges): {}",
                    err
                )
            }
            ErrorKind::CannotDeleteVariableGlobally(err) => {
                write!(
                    f,
                    "Can't delete variable globally (try running with sudo or administrative privileges): {}",
                    err
                )
            }
            ErrorKind::ParsingError(err) => write!(f, "Parsing error: {}", err),
            ErrorKind::FileError(err) => write!(f, "File error: {}", err),
            ErrorKind::CannotFindVariable(name, _) => write!(f, "Can't find variable: {}", name),
            ErrorKind::NameValidationError(err) => write!(f, "Name validation error: {}", err),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    /// Format, used to print variables using print command
    pub print_format: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind_display() {
        let test_cases = vec![
            (ErrorKind::StartingProcessError, "Can't start process"),
            (
                ErrorKind::CannotSetVariableGlobally("Permission denied".to_string()),
                "Can't set variable globally (try running with sudo or administrative privileges): Permission denied",
            ),
            (
                ErrorKind::CannotDeleteVariableGlobally("Access denied".to_string()),
                "Can't delete variable globally (try running with sudo or administrative privileges): Access denied",
            ),
            (
                ErrorKind::ParsingError("Invalid syntax".to_string()),
                "Parsing error: Invalid syntax",
            ),
            (
                ErrorKind::FileError("File not found".to_string()),
                "File error: File not found",
            ),
            (
                ErrorKind::CannotFindVariable("PATH".to_string(), true),
                "Can't find variable: PATH",
            ),
            (
                ErrorKind::NameValidationError("Variable name cannot be empty".to_string()),
                "Name validation error: Variable name cannot be empty",
            ),
        ];

        for (error, expected) in test_cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn test_config_parsing_error_display() {
        let test_cases = vec![
            (
                ConfigParsingError::FSError("Permission denied".to_string()),
                "Error while reading file: Permission denied",
            ),
            (
                ConfigParsingError::ParsingError("Invalid JSON".to_string()),
                "Error while parsing file: Invalid JSON",
            ),
            (
                ConfigParsingError::FileDoesntExists,
                "Config file doesn't exists",
            ),
        ];

        for (error, expected) in test_cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
