pub mod secure;
pub mod encoder;
pub mod validator;
pub mod clipboard;

use clap::{Args, Parser, Subcommand, crate_version};
use crate::secure::secure_pass;
use crate::validator::validate_generator_args;
use crate::clipboard::copy;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a 'secure' random alphanumeric string
    Generate(GenerateArgs),

    /// Print the version and exit
    Version,
}

#[derive(Args)]
pub struct GenerateArgs {
    ///  Character length
    #[arg(default_value_t = 20)]
    length: u16,

    /// send to your clipboard instead of STDOUT
    #[arg(long, short='c', default_value_t = false)]
    clipboard: bool,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Generate(gen_args) => {
            match validate_generator_args(gen_args) {
                Ok(valid) => {
                    let password =  secure_pass(valid.length);
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