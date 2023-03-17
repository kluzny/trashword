pub mod secure;
pub mod encoder;

use clap::{Args, Parser, Subcommand, crate_version};
use crate::secure::generate_password;

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
struct GenerateArgs {
    ///  Character length
    #[arg(default_value_t = 20)]
    length: u16,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Generate(gen_args) => {
            match validate_generator_args(gen_args) {
                Ok(valid) => { generate_password(valid.length) },
                Err(message) => { panic!("{message}") }
            }
        },
        Commands::Version => {
            println!(crate_version!())
        }
    }
}

fn validate_generator_args(gen_args: &GenerateArgs) -> Result<&GenerateArgs, String> {
    let length = gen_args.length;
    if length < 1 || length > 256 {
        Err(String::from("length should be between 1 and 256"))
    } else {
        Ok(gen_args)
    }
}