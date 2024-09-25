use clap::Args;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
#[command(name = "rosenpass")]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Exchange a config file with peers
    ExchangeConfig(ExchangeConfig),

    /// Start in daemon mode, performing key exchanges
    Exchange(Exchange),

    /// Generate a new config file
    GenConfig(GenConfig),

    /// Generate new keys
    GenKeys(GenKeys),

    /// Validate one or more config files
    Validate(Validate),

    /// Show man page
    Man(Man),
}

#[derive(Args)]
pub struct ExchangeConfig {
    /// The config file to exchange
    #[clap(short, long)]
    pub config_file: PathBuf,
}

#[derive(Args)]
pub struct Exchange {
    /// public-key <PATH> secret-key <PATH> [listen <ADDR>:<PORT>]... [verbose]
    #[clap(value_name = "OWN_CONFIG")]
    first_arg: String,

    /// peer public-key <PATH> [ENDPOINT] [PSK] [OUTFILE] [WG]
    ///
    /// ENDPOINT := endpoint <HOST/IP>:<PORT>
    ///
    /// PSK := preshared-key <PATH>
    ///
    /// OUTFILE := outfile <PATH>
    ///
    /// WG := wireguard <WIREGUARD_DEV> <WIREGUARD_PEER> [WIREGUARD_EXTRA_ARGS]...
    #[clap(value_name = "PEERS")]
    rest_of_args: Vec<String>,

    #[clap(short, long)]
    config_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct GenConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,

    #[clap(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct GenKeys {
    #[clap(short, long)]
    pub config_file: Option<PathBuf>,

    #[clap(short, long)]
    pub public_key: Option<PathBuf>,

    #[clap(short, long)]
    pub secret_key: Option<PathBuf>,

    #[clap(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct Validate {
    #[clap(short, long)]
    pub config_files: Vec<PathBuf>,
}

#[derive(Args)]
pub struct Man {}
