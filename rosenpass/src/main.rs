mod cmd;

use clap::Parser;
use cmd::Command;
use rosenpass::cli::{Cli, Commands};

pub fn main() -> anyhow::Result<()> {
    {
        use rosenpass_secret_memory as SM;
        #[cfg(feature = "experiment_memfd_secret")]
        SM::secret_policy_try_use_memfd_secrets();
        #[cfg(not(feature = "experiment_memfd_secret"))]
        SM::secret_policy_use_only_malloc_secrets();
    }

    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match cli.command {
        Commands::ExchangeConfig(_) => todo!(),
        Commands::Exchange(_) => todo!(),
        Commands::GenConfig(genconfig) => genconfig.run(),
        Commands::GenKeys(genkeys) => genkeys.run(),
        Commands::Validate(_) => todo!(),
        Commands::Man(_) => todo!(),
    }

    // let broker_interface = args.get_broker_interface();
    // match args.run(broker_interface, None) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         error!("{e:?}");
    //         exit(1);
    //     }
    // }
}
