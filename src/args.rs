use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file, defaults to `config.toml`
    #[arg(short, long, default_value = "config.toml")]
    pub config_file: String,

    /// Prints the configuration to the console and exits
    #[arg(short = 'd', long)]
    pub dump_config: bool,

    /// Prints the configuration to the console and exits
    #[arg(short = 'D', long)]
    pub dump_default_config: bool,
}
