use clap::{Args};

#[derive(Args)]
pub struct AuthArgs {
    /// A domain or hostname
    #[arg(default_value_t = String::new())]
    pub domain: String,

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