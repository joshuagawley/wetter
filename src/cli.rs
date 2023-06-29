use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Location to use, if none, defaults to using user's IP to determine current location
    #[arg(short, long)]
    pub(crate) location: Option<String>,
}
