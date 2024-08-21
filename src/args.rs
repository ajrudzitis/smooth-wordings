use clap::Parser;

/// An art project of smooth wordings
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the key file to load
    #[arg(short, long)]
    key: String,
}