use std::path::PathBuf;

use clap::Args;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(name = "rosenpass")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ExchangeConfig(ExchangeConfig),
    GenConfig(GenConfig),
}

#[derive(Debug, Args)]
pub struct ExchangeConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,
}

#[derive(Debug, Args)]
pub struct GenConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,
    pub force: bool,
}
