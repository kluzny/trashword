use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a deterministic password from the supplied secret
    Auth(AuthArgs),

    /// Generate a 'secure' random alphanumeric string
    Generate(GenerateArgs),

    /// Print the version and exit
    Version,
}


#[derive(Args)]
pub struct AuthArgs {
    /// Optional domain or hostname
    #[arg(short='d', long)]
    pub domain: Option<String>,

    /// Optional user or username
    #[arg(short='u', long)]
    pub user: Option<String>,

    /// send to your clipboard instead of STDOUT
    #[arg(long, short='c', default_value_t = false)]
    pub clipboard: bool,
}

#[derive(Args)]
pub struct GenerateArgs {
    ///  Character length
    #[arg(default_value_t = 20)]
    pub length: u16,

    /// send to your clipboard instead of STDOUT
    #[arg(long, short='c', default_value_t = false)]
    pub clipboard: bool,
}