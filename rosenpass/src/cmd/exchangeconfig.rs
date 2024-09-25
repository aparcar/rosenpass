use super::Command;
use anyhow::Result;
use rosenpass::cli;

impl Command for cli::ExchangeConfig {
    fn run(self) -> Result<()> {
        println!("ExchangeConfig");
        Ok(())
    }
}
