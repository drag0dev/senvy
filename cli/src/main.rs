pub mod args_structure;
pub mod config;
pub mod command_handlers;
pub mod utils;
pub mod types;

use clap::Parser;
use args_structure::Commands;
use command_handlers::*;

// TODO: colored
// TODO: are all prints giving enough info
// TODO: spell check

#[tokio::main]
async fn main() {
    let args = args_structure::Cli::parse();
    let config = config::read_config();
    if config.is_err() {
        println!("Error during initial config reading: {}",
            config.as_ref().err().unwrap());
    }

    let config = config.unwrap();
    let res = match args.command {
        Commands::Init{name, file, remote_url} => init(config, name, file, remote_url).await,
        Commands::New{name, file, remote_url} => new(config, name, file, remote_url).await,
        Commands::Delete{name, remote_url} => delete(config, name, remote_url).await,
        Commands::Pull{name, remote_url} => pull(config, name, remote_url).await,
        Commands::Push{name, file, remote_url} => push(config, name, file, remote_url).await,
        Commands::Check{} => check(config).await,
    };

    if res.is_err() {
        let err = res.err().unwrap();
        println!("Error encountered while: {}", err);
        println!("\t{}", err.root_cause());
    }
}
