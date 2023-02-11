pub mod args_structure;
pub mod config;
pub mod command_handlers;

use clap::Parser;
use args_structure::Commands;
use command_handlers::*;

fn main() {
    let args = args_structure::Cli::parse();
    let config = config::read_config();
    if config.is_err() {
        println!("Error during initial config reading: {}",
            config.as_ref().err().unwrap());
    }

    let config = config.unwrap();
    let res = match args.command {
        Commands::Init{name, file, remote_url} => init(config, name, file, remote_url),
        Commands::New{name, file, remote_url} => new(config, name, file, remote_url),
        Commands::Delete{name, remote_url} => delete(config, name, remote_url),
        Commands::Pull{name, remote_url} => pull(config, name, remote_url),
        Commands::Push{name, file, remote_url} => push(config, name, file, remote_url),
        Commands::Check{} => check(config),
    };
}
