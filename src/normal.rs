//! chains.json

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

pub use crate::common::*;

/// A [Chain]'s ENS registry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ens {
    /// The address of the ENS registry address.
    pub registry: String,
}

/// A [Chain]'s blockchain explorer.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Explorer {
    /// The name of the blockchain explorer.
    pub name: String,
    /// The URL of the blockchain explorer.
    pub url: String,
    /// The standard of the blockchain explorer.
    pub standard: String,
}

/// A [Chain]'s bridge.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bridge {
    /// The URL of the bridge.
    pub url: String,
}

/// A [Chain]'s parent network.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parent {
    /// The type of the child network.
    #[serde(rename = "type")]
    pub type_: String,
    /// The name of the parent network.
    pub chain: String,
    /// The bridges between the parent and the child network, if any.
    pub bridges: Option<Vec<Bridge>>,
}

/// A [Chain]'s red flag.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RedFlag {
    /// The network reuses a previous network's chain ID.
    ReusedChainId,
}

/// EIP-155 Chain Data.
///
/// Schema: <https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    /// The name of the network.
    pub name: String,

    /// The short name of the network.
    ///
    /// Matches the pattern: `^[A-Za-z0-9-_]{1,64}$`
    pub short_name: String,

    /// An optional title for the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The name of the network.
    pub chain: String,

    /// The icon type of the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The RPCs of the network, if any.
    pub rpc: Vec<String>,

    /// The faucet URLs of the network, if any.
    pub faucets: Vec<String>,

    /// The native currency of the network.
    pub native_currency: NativeCurrency,

    /// The info URL of the network.
    #[serde(rename = "infoURL")]
    pub info_url: String,

    /// The chain ID of the network.
    pub chain_id: u64,

    /// The network ID of the network.
    pub network_id: u64,

    /// The optional slip44 of the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slip44: Option<u64>,

    /// The ENS registry address of the network, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ens: Option<Ens>,

    /// The block explorers of the network, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explorers: Option<Vec<Explorer>>,

    /// The optional parent of the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,

    /// Whether the status is deprecated, incubating or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The red flags of the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red_flags: Option<Vec<RedFlag>>,
}
