use super::Command;
use anyhow::Result;
use rosenpass::cli;
use rosenpass::config::Rosenpass;

impl Command for cli::Validate {
    fn run(self) -> Result<()> {
        eprintln!("Validate config files");
        for file in self.config_files {
            match Rosenpass::load(file.clone()) {
                Ok(config) => {
                    eprintln!("{file:?} is valid TOML and conforms to the expected schema");
                    match config.validate() {
                        Ok(_) => eprintln!("{file:?} has passed all logical checks"),
                        Err(_) => eprintln!("{file:?} contains logical errors"),
                    }
                }
                Err(e) => eprintln!("{file:?} is not valid: {e}"),
            }
        }

        Ok(())
    }
}
