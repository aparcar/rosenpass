use std::path::PathBuf;

use clap::Args;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
#[command(name = "rosenpass")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ExchangeConfig(ExchangeConfig),
    GenConfig(GenConfig),
}

#[derive(Args)]
pub struct ExchangeConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,
}

#[derive(Args)]
pub struct GenConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,
    #[clap(short, long)]
    pub force: bool,
}
