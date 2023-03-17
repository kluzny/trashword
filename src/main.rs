pub mod auth;
pub mod clipboard;
pub mod commands;
pub mod encoder;
pub mod secure;
pub mod validator;

use clap::{Parser, crate_version};
use crate::auth::build_credentials;
use crate::clipboard::copy;
use crate::commands::Commands;
use crate::secure::generate_password;
use crate::validator::validate_generator_args;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Increase the verbosity
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Auth(auth_args) => {
            build_credentials(auth_args);
        },
        Commands::Generate(gen_args) => {
            match validate_generator_args(gen_args) {
                Ok(valid) => {
                    let password =  generate_password(valid.length);
                    if valid.clipboard {
                        copy(password);
                    } else {
                        println!("{}", password);
                    }
                },
                Err(message) => { panic!("{message}") }
            }
        },
        Commands::Version => {
            println!(crate_version!())
        }
    }
}