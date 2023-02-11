use clap::{Subcommand, Parser};

#[derive(Debug, Parser)]
#[command(name = "senvy")]
#[command(about = "cli for server thath keeps your env vars", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "initialize senvy in your project")]
    Init {},

    #[command(about = "create a new project entry on the server")]
    New {
        #[arg(value_name = "project name")]
        name: String,
        #[arg(value_name = "path to the file with env vars",
            require_equals = true)]
        file: String
    },

    #[command(about = "delete project entry on the server")]
    Delete {
        #[arg(value_name = "project name")]
        name: String
    },

    #[command(about = "pull env vars from the server")]
    Pull {
        #[arg(value_name = "project name")]
        name: String
    },

    #[command(about = "push env vars to the server")]
    Push {
        #[arg(value_name = "project name")]
        name: String,
        #[arg(value_name = "path to the file with env vars",
            require_equals = true)]
        file: String
    },

    // TODO: implement endpoint
    #[command(about = "check if there are new env vars available")]
    Check {
        #[arg(value_name = "project name")]
        name: String
    }
}
