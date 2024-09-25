use super::Command;
use anyhow::Result;
use rosenpass::cli;

impl Command for cli::Man {
    fn run(self) -> Result<()> {
        let man_cmd = std::process::Command::new("man")
            .args(["1", "rosenpass"])
            .status();

        if !(man_cmd.is_ok() && man_cmd.unwrap().success()) {
            println!(include_str!(env!("ROSENPASS_MAN")));
        }

        Ok(())
    }
}
