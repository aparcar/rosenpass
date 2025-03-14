#![cfg(target_os = "linux")]
//! Linux-specific WireGuard PSK broker implementation using netlink.
//!
//! This module provides direct kernel communication through netlink sockets for managing
//! WireGuard pre-shared keys. It's more efficient than the command-line implementation
//! but only available on Linux systems.
//!
//! # Examples
//!
//! ```no_run
//! use rosenpass_secret_memory::{Public, Secret};
//! use rosenpass_wireguard_broker::{WireGuardBroker, SerializedBrokerConfig, WG_KEY_LEN, WG_PEER_LEN};
//! use rosenpass_wireguard_broker::brokers::netlink::NetlinkWireGuardBroker;
//! # use rosenpass_secret_memory::secret_policy_use_only_malloc_secrets;
//! # secret_policy_use_only_malloc_secrets();
//!
//! let mut broker = NetlinkWireGuardBroker::new()?;
//!
//! let config = SerializedBrokerConfig {
//!     interface: "wg0".as_bytes(),
//!     peer_id: &Public::zero(), // Replace with actual peer ID
//!     psk: &Secret::zero(),     // Replace with actual PSK
//!     additional_params: &[],
//! };
//!
//! broker.set_psk(config)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use std::fmt::Debug;

use wireguard_uapi::linux as wg;

use crate::api::config::NetworkBrokerConfig;
use crate::api::msgs;
use crate::{SerializedBrokerConfig, WireGuardBroker};

/// Error that can occur when connecting to the WireGuard netlink interface.
#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error(transparent)]
    ConnectError(#[from] wg::err::ConnectError),
}

/// Errors that can occur during netlink operations.
#[derive(thiserror::Error, Debug)]
pub enum NetlinkError {
    #[error(transparent)]
    SetDevice(#[from] wg::err::SetDeviceError),
    #[error(transparent)]
    GetDevice(#[from] wg::err::GetDeviceError),
}

/// Errors that can occur when setting a pre-shared key.
#[derive(thiserror::Error, Debug)]
pub enum SetPskError {
    #[error("The indicated wireguard interface does not exist")]
    NoSuchInterface,
    #[error("The indicated peer does not exist on the wireguard interface")]
    NoSuchPeer,
    #[error(transparent)]
    NetlinkError(#[from] NetlinkError),
}

/// # Example
/// ```
/// # use wireguard_uapi::err::NlError;
/// # use wireguard_uapi::linux::err::SetDeviceError;
/// use rosenpass_wireguard_broker::brokers::netlink::SetPskError;
/// let set_device_error: SetDeviceError = SetDeviceError::NlError(NlError::Msg("test-error".to_string()));
/// let set_psk_error: SetPskError = set_device_error.into();
/// ```
impl From<wg::err::SetDeviceError> for SetPskError {
    fn from(err: wg::err::SetDeviceError) -> Self {
        NetlinkError::from(err).into()
    }
}

/// # Example
/// ```
/// # use wireguard_uapi::err::NlError;
/// # use wireguard_uapi::linux::err::GetDeviceError;
/// # use rosenpass_wireguard_broker::brokers::netlink::SetPskError;
/// let get_device_error: GetDeviceError = GetDeviceError::NlError(NlError::Msg("test-error".to_string()));
/// let set_psk_error: SetPskError = get_device_error.into();
/// ```
impl From<wg::err::GetDeviceError> for SetPskError {
    fn from(err: wg::err::GetDeviceError) -> Self {
        NetlinkError::from(err).into()
    }
}

/// # Example
/// ```
/// use rosenpass_wireguard_broker::api::msgs::SetPskError as SetPskMsgsError;
/// use rosenpass_wireguard_broker::brokers::netlink::SetPskError as SetPskNetlinkError;
/// let set_psk_nlink_error: SetPskNetlinkError = SetPskNetlinkError::NoSuchInterface;
/// let set_psk_msgs_error = SetPskMsgsError::from(set_psk_nlink_error);
/// ```
use msgs::SetPskError as SetPskMsgsError;
use SetPskError as SetPskNetlinkError;
impl From<SetPskNetlinkError> for SetPskMsgsError {
    fn from(err: SetPskError) -> Self {
        match err {
            SetPskNetlinkError::NoSuchPeer => SetPskMsgsError::NoSuchPeer,
            _ => SetPskMsgsError::InternalError,
        }
    }
}

/// WireGuard broker implementation using Linux netlink sockets.
///
/// This implementation communicates directly with the kernel through netlink sockets,
/// providing better performance than command-line based implementations.
///
/// # Examples
///
/// ```
/// use rosenpass_wireguard_broker::brokers::netlink::NetlinkWireGuardBroker;
/// use rosenpass_wireguard_broker::WireGuardBroker;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut broker = NetlinkWireGuardBroker::new()?;
/// # Ok(())
/// # }
/// ```
///
/// # Platform Support
///
/// This implementation is only available on Linux systems and requires appropriate
/// permissions to use netlink sockets.
pub struct NetlinkWireGuardBroker {
    sock: wg::WgSocket,
}

impl NetlinkWireGuardBroker {
    /// Opens a netlink socket to the WireGuard kernel module
    /// and returns a new netlink-based WireGuard broker.
    pub fn new() -> Result<Self, ConnectError> {
        let sock = wg::WgSocket::connect()?;
        Ok(Self { sock })
    }
}

impl Debug for NetlinkWireGuardBroker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Add useful info in Debug
        f.debug_struct("NetlinkWireGuardBroker").finish()
    }
}

impl WireGuardBroker for NetlinkWireGuardBroker {
    type Error = SetPskError;

    fn set_psk(&mut self, config: SerializedBrokerConfig) -> Result<(), Self::Error> {
        let config: NetworkBrokerConfig = config
            .try_into()
            // TODO: I think this is the wrong error
            .map_err(|_e| SetPskError::NoSuchInterface)?;
        // Ensure that the peer exists by querying the device configuration
        // TODO: Use InvalidInterfaceError

        let state = self
            .sock
            .get_device(wg::DeviceInterface::from_name(config.iface))?;

        if !state
            .peers
            .iter()
            .any(|p| p.public_key == config.peer_id.value)
        {
            return Err(SetPskError::NoSuchPeer);
        }

        // Peer update description
        let mut set_peer = wireguard_uapi::set::Peer::from_public_key(config.peer_id);
        set_peer
            .flags
            .push(wireguard_uapi::linux::set::WgPeerF::UpdateOnly);
        set_peer.preshared_key = Some(config.psk.secret());

        // Device update description
        let mut set_dev = wireguard_uapi::set::Device::from_ifname(config.iface);
        set_dev.peers.push(set_peer);

        self.sock.set_device(set_dev)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rosenpass_secret_memory::{secret_policy_use_only_malloc_secrets, Public, Secret};
    #[test]
    fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
        secret_policy_use_only_malloc_secrets();
        let result = NetlinkWireGuardBroker::new();
        assert!(result.is_ok());
        let mut broker = result.unwrap();
        let peer_id = Public::zero();
        let psk = Secret::zero();
        let config: SerializedBrokerConfig = NetworkBrokerConfig {
            iface: "wg0",
            peer_id: &peer_id,
            psk: &psk,
        }
        .into();
        let result = broker.set_psk(config);
        assert!(result.is_err());
        Ok(())
    }
}
