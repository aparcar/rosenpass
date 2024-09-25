use super::Command;
use anyhow::ensure;
use anyhow::Result;
use rosenpass::cli;
use rosenpass::config;

impl Command for cli::GenConfig {
    fn run(self) -> Result<()> {
        println!("GenConfig");
        ensure!(
            self.force || !self.config_file.exists(),
            "config file {0:?} already exists",
            self.config_file
        );

        config::Rosenpass::example_config().store(self.config_file)
    }
}
