use super::Command;
use anyhow::Result;
use rosenpass::cli;

impl Command for cli::Exchange {
    fn run(self) -> Result<()> {
        println!("Exchange");
        Ok(())
    }
}
