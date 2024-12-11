use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    after_help = "Get more info at project's repo: https://github.com/ankddev/envfetch",
    after_long_help = "Get more info at project's GitHub repo available at https://github.com/ankddev/envfetch",
    arg_required_else_help = true
)]
#[command(
    about = "envfetch - lightweight tool for working with environment variables",
    long_about = "envfetch is a lightweight cross-platform CLI tool for working with environment variables"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Exit on any error
    #[arg(long, short = 'e', global = true)]
    pub exit_on_error: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Prints value of environment variable
    Get(GetArgs),
    /// Set environment variable and run given process
    Set(SetArgs),
    /// Set environment variable permanently
    Gset(GsetArgs),
    /// Delete environment variable and run given process
    Delete(DeleteArgs),
    /// Delete environment variable permanently
    Gdelete(GdeleteArgs),
    /// Load environment variables from dotenv file and run process
    Load(LoadArgs),
    /// Load environment variables from dotenv file permanently
    Gload(GloadArgs),
    /// Prints all environment variables
    Print,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Disable showing similar variables' names if variable not found
    #[arg(long, short = 's', default_value = "false")]
    pub no_similar_names: bool,
}

#[derive(Args, Debug)]
pub struct LoadArgs {
    /// Process to start
    #[arg(required = true)]
    pub process: String,
    /// Path to .env file
    #[arg(long, short, default_value = ".env")]
    pub file: String,
}

#[derive(Args, Debug)]
pub struct SetArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Value for environment variable
    #[arg(required = true)]
    pub value: String,
    /// Process to start
    #[arg(required = true)]
    pub process: String,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Process to start
    #[arg(required = true)]
    pub process: String,
}

#[derive(Args, Debug)]
pub struct GsetArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
    /// Value for environment variable
    #[arg(required = true)]
    pub value: String,
}

#[derive(Args, Debug)]
pub struct GdeleteArgs {
    /// Environment variable name
    #[arg(required = true)]
    pub key: String,
}

#[derive(Args, Debug)]
pub struct GloadArgs {
    /// Path to .env file
    #[arg(long, short, default_value = ".env")]
    pub file: String,
} 