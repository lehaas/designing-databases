use clap::{command, Parser};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(long)]
    key: String,
    /// The path to the file to read
    #[arg(long)]
    value: String,
    /// Verbosity of the logs
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> () {
    let _: Cli = Cli::parse();
}
