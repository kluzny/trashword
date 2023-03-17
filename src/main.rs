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
            if validate_generator_args(gen_args) {
                generate_password(gen_args.length);
            }
        },
        Commands::Version => {
            println!(crate_version!())
        }
    }
}

// TODO: switch to -> Result<Ok, Err(String)>
fn validate_generator_args(gen_args: &GenerateArgs) -> bool {
    let length = gen_args.length;
    if length < 1 || length > 256 {
        println!("length should be between 1 and 256");
        false
    } else {
        true
    }
}