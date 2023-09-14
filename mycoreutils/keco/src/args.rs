use clap::Parser;
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct KecoArgs {
    /// color in hex format
    #[arg(long, short)]
    pub color: String,

    pub text: Vec<String>,
}
