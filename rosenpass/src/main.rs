mod cmd;

use clap::Parser;
use cmd::Command;
use rosenpass::cli::{Cli, Commands};

/// Catches errors, prints them through the logger, then exits
pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        // Commands::ExchangeConfig(_) => println!("ExchangeConfig"),
        Commands::GenConfig(_) => {
            println!("GenConfig");
        }
        Commands::ExchangeConfig(_) => todo!(),
    }

    // {
    //     use rosenpass_secret_memory as SM;
    //     #[cfg(feature = "experiment_memfd_secret")]
    //     SM::secret_policy_try_use_memfd_secrets();
    //     #[cfg(not(feature = "experiment_memfd_secret"))]
    //     SM::secret_policy_use_only_malloc_secrets();
    // }

    // // init logging
    // {
    //     let mut log_builder = env_logger::Builder::from_default_env(); // sets log level filter from environment (or defaults)
    //     if let Some(level) = args.get_log_level() {
    //         log::debug!("setting log level to {:?} (set via CLI parameter)", level);
    //         log_builder.filter_level(level); // set log level filter from CLI args if available
    //     }
    //     log_builder.init();

    //     // // check the effectiveness of the log level filter with the following lines:
    //     // use log::{debug, error, info, trace, warn};
    //     // trace!("trace dummy");
    //     // debug!("debug dummy");
    //     // info!("info dummy");
    //     // warn!("warn dummy");
    //     // error!("error dummy");
    // }

    // let broker_interface = args.get_broker_interface();
    // match args.run(broker_interface, None) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         error!("{e:?}");
    //         exit(1);
    //     }
    // }
}
