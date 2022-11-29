//! chains_mini.json

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

pub use crate::common::*;

/// EIP-155 Chain Data.
///
/// [Mini version]: includes only the following fields:
/// - name
/// - chainId
/// - shortName
/// - networkId
/// - nativeCurrency
/// - rpc
/// - faucets
/// - infoURL
///
/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
///
/// [Mini version]: https://github.com/ethereum-lists/chains/blob/2bd5eb7a2f7796411ee13def4b4e601f77f9c243/processor/src/main/kotlin/org/ethereum/lists/chains/Main.kt#L58-L64
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainData {
    /// Name of the network.
    pub name: String,

    /// Chain ID of the Network.
    pub chain_id: u64,

    /// Matches the pattern: `^[A-Za-z0-9-_]{1,64}$`
    pub short_name: String,

    /// Network ID of the Network.
    pub network_id: u64,

    pub native_currency: NativeCurrency,

    pub rpc: Vec<String>,

    pub faucets: Vec<String>,

    #[serde(rename = "infoURL")]
    pub info_url: String,
}
