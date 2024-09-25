use super::Command;
use anyhow::Result;
use rosenpass::cli;

impl Command for cli::ExchangeConfig {
    fn run(self) -> Result<()> {
        eprintln!("ExchangeConfig");
        ensure!(
            self.config_file.exists(),
            "config file '{config_file:?}' does not exist"
        );

        let mut config = config::Rosenpass::load(self.config_file)?;
        config.validate()?;
        self.apply_to_config(&mut config)?;
        config.check_usefullness()?;

        Self::event_loop(config, broker_interface, test_helpers)?;
        Ok(())
    }
}
