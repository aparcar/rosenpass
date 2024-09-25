use super::Command;
use anyhow::Result;
use rosenpass::cli;

impl Command for cli::Exchange {
    fn run(self) -> Result<()> {
        eprintln!("Exchange");
        let mut rest_of_args = self.rest_of_args.clone();
        rest_of_args.insert(0, self.first_arg.clone());
        let args = rest_of_args;
        let mut config = config::Rosenpass::parse_args(args)?;

        if let Some(p) = self.config_file {
            config.store(p)?;
            config.config_file_path.clone_from(p);
        }
        config.validate()?;
        self.apply_to_config(&mut config)?;
        config.check_usefullness()?;

        Self::event_loop(config, broker_interface, test_helpers)?;
        Ok(())
    }
}
