use clap::{Subcommand, Parser};

#[derive(Debug, Parser)]
#[command(name = "senvy")]
#[command(about = "cli for senvy, server thath keeps your env vars", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "initialize senvy in your project")]
    Init {
        #[arg(value_name = "project name")]
        name: String,

        #[arg(value_name = "path to the file with env vars")]
        file: String,

        #[arg(value_name = "server url")]
        remote_url: String
    },

    #[command(about = "create a new project entry on the server")]
    New {
        #[arg(value_name = "project name")]
        name: String,

        #[arg(value_name = "path to the file with env vars")]
        file: String,

        #[arg(value_name = "server url")]
        remote_url: String,
    },

    #[command(about = "delete project entry on the server, blank means current project")]
    Delete {
        #[arg(value_name = "project name")]
        name: Option<String>,

        #[arg(value_name = "server url")]
        remote_url: Option<String>,
    },

    #[command(about = "pull env vars from the server, blank means current project")]
    Pull {
        #[arg(value_name = "project name")]
        name: Option<String>,

        #[arg(value_name = "server url")]
        remote_url: Option<String>,
    },

    #[command(about = "push env vars to the server, blank name current project")]
    Push {
        #[arg(value_name = "project name")]
        name: Option<String>,

        #[arg(value_name = "path to the file with env vars")]
        file: Option<String>,

        #[arg(value_name = "server url")]
        remote_url: Option<String>,
    },

    #[command(about = "check if there are new env vars available")]
    Check {}
}
