pub mod secure;
pub mod encoder;
pub mod validator;
pub mod clipboard;
pub mod commands;

use clap::{Parser, Subcommand, crate_version};
use crate::secure::generate_password;
use crate::validator::validate_generator_args;
use crate::clipboard::copy;
use crate::commands::{AuthArgs, GenerateArgs};

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Increase the verbosity
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a deterministic password from the supplied secret
    Auth(AuthArgs),

    /// Generate a 'secure' random alphanumeric string
    Generate(GenerateArgs),

    /// Print the version and exit
    Version,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Auth(auth_args) => {
            // TODO hide behind versbose
            if auth_args.domain.len() > 0 {
                println!("Generating password for {}...", auth_args.domain);
            } else {
                println!("Generating password ...");
            }
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