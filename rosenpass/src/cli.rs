use crate::app_server::AppServerTest;
use crate::app_server::{AppServer, BrokerPeer};
use crate::protocol::{SPk, SSk, SymKey};
use clap::Args;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use rosenpass_cipher_traits::Kem;
use rosenpass_ciphers::kem::StaticKem;
use rosenpass_secret_memory::file::StoreSecret;
use rosenpass_util::file::{LoadValue, LoadValueB64, StoreValue};

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
#[command(name = "rosenpass")]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Exchange a config file with peers
    ExchangeConfig(ExchangeConfig),

    /// Start in daemon mode, performing key exchanges
    Exchange(Exchange),

    /// Generate a new config file
    GenConfig(GenConfig),

    /// Generate new keys
    GenKeys(GenKeys),

    /// Validate one or more config files
    Validate(Validate),

    /// Show man page
    Man(Man),
}

#[derive(Args)]
pub struct ExchangeConfig {
    /// The config file to exchange
    #[clap(short, long)]
    pub config_file: PathBuf,
}

#[derive(Args)]
pub struct Exchange {
    /// public-key <PATH> secret-key <PATH> [listen <ADDR>:<PORT>]... [verbose]
    #[clap(value_name = "OWN_CONFIG")]
    pub first_arg: String,

    /// peer public-key <PATH> [ENDPOINT] [PSK] [OUTFILE] [WG]
    ///
    /// ENDPOINT := endpoint <HOST/IP>:<PORT>
    ///
    /// PSK := preshared-key <PATH>
    ///
    /// OUTFILE := outfile <PATH>
    ///
    /// WG := wireguard <WIREGUARD_DEV> <WIREGUARD_PEER> [WIREGUARD_EXTRA_ARGS]...
    #[clap(value_name = "PEERS")]
    pub rest_of_args: Vec<String>,

    #[clap(short, long)]
    pub config_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct GenConfig {
    #[clap(short, long)]
    pub config_file: PathBuf,

    #[clap(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct GenKeys {
    #[clap(short, long)]
    pub config_file: Option<PathBuf>,

    #[clap(short, long)]
    pub public_key: Option<PathBuf>,

    #[clap(short, long)]
    pub secret_key: Option<PathBuf>,

    #[clap(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct Validate {
    #[clap(short, long)]
    pub config_files: Vec<PathBuf>,
}

pub enum BrokerInterface {
    Socket(PathBuf),
    FileDescriptor(i32),
    SocketPair,
}

#[derive(Args)]
pub struct Man {}

// fn event_loop(
//     config: config::Rosenpass,
//     broker_interface: Option<BrokerInterface>,
//     test_helpers: Option<AppServerTest>,
// ) -> anyhow::Result<()> {
//     const MAX_PSK_SIZE: usize = 1000;

//     // load own keys
//     let keypair = config
//         .keypair
//         .as_ref()
//         .map(|kp| -> anyhow::Result<_> {
//             let sk = SSk::load(&kp.secret_key)?;
//             let pk = SPk::load(&kp.public_key)?;
//             Ok((sk, pk))
//         })
//         .transpose()?;

//     // start an application server
//     let mut srv = std::boxed::Box::<AppServer>::new(AppServer::new(
//         keypair,
//         config.listen.clone(),
//         config.verbosity,
//         test_helpers,
//     )?);

//     config.apply_to_app_server(&mut srv)?;

//     let broker = Self::create_broker(broker_interface)?;
//     let broker_store_ptr = srv.register_broker(broker)?;

//     fn cfg_err_map(e: NativeUnixBrokerConfigBaseBuilderError) -> anyhow::Error {
//         anyhow::Error::msg(format!("NativeUnixBrokerConfigBaseBuilderError: {:?}", e))
//     }

//     for cfg_peer in config.peers {
//         let broker_peer = if let Some(wg) = &cfg_peer.wg {
//             let peer_cfg = NativeUnixBrokerConfigBaseBuilder::default()
//                 .peer_id_b64(&wg.peer)?
//                 .interface(wg.device.clone())
//                 .extra_params_ser(&wg.extra_params)?
//                 .build()
//                 .map_err(cfg_err_map)?;

//             let broker_peer = BrokerPeer::new(broker_store_ptr.clone(), Box::new(peer_cfg));

//             Some(broker_peer)
//         } else {
//             None
//         };

//         srv.add_peer(
//             // psk, pk, outfile, outwg, tx_addr
//             cfg_peer
//                 .pre_shared_key
//                 .map(SymKey::load_b64::<MAX_PSK_SIZE, _>)
//                 .transpose()?,
//             SPk::load(&cfg_peer.public_key)?,
//             cfg_peer.key_out,
//             broker_peer,
//             cfg_peer.endpoint.clone(),
//         )?;
//     }

//     srv.event_loop()
// }
