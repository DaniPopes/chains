//! chains.json

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

pub use crate::common::*;

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ens {
    /// ENS registry address.
    pub registry: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Explorer {
    pub name: String,
    pub url: String,
    pub standard: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bridge {
    pub url: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_value: String,
    pub chain: String,
    pub bridges: Option<Vec<Bridge>>,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RedFlag {
    ReusedChainId,
}

/// EIP-155 Chain Data.
///
/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    /// Name of the network.
    pub name: String,

    /// Matches the pattern: `^[A-Za-z0-9-_]{1,64}$`
    pub short_name: String,

    /// Optional title for the network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Name of the network.
    pub chain: String,

    /// Icon type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    pub rpc: Vec<String>,

    pub faucets: Vec<String>,

    pub native_currency: NativeCurrency,

    #[serde(rename = "infoURL")]
    pub info_url: String,

    /// Chain ID of the Network.
    pub chain_id: u64,

    /// Network ID of the Network.
    pub network_id: u64,

    /// Slip44 of the Network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slip44: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ens: Option<Ens>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explorers: Option<Vec<Explorer>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red_flags: Option<Vec<RedFlag>>,
}
