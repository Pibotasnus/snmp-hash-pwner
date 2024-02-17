use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path to wordlist file containing passwords
    #[arg(short, long)]
    pub dict_file: String,

    /// Path to hash extracted from an SNMP capture
    #[arg(short, long)]
    pub target_file: String,
}
