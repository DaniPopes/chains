//! chains_mini.json

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    /// Name of the native currency.
    pub name: String,
    /// Symbol of the native currency.
    pub symbol: String,
    /// Decimals of the native currency.
    pub decimals: u8,
}

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

    pub chain_id: u64,

    pub short_name: String,

    pub network_id: u64,

    pub rpc: Vec<String>,

    pub faucets: Vec<String>,

    pub native_currency: NativeCurrency,

    #[serde(rename = "infoURL")]
    pub info_url: String,
}
