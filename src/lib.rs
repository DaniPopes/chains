//! # chains

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CHAIN_DATA: Lazy<Vec<ChainData>> = Lazy::new(|| {
    #[cfg(not(feature = "zip"))]
    {
        const S: &str = include_str!("../data/chains.json");
        serde_json::from_str(S).unwrap()
    }
    #[cfg(feature = "zip")]
    {
        const Z: &[u8] = include_bytes!("../data/chains.zip");
        let reader = std::io::Cursor::new(Z);
        let mut archive = zip::ZipArchive::new(reader).unwrap();
        let file = archive.by_index(0).unwrap();
        serde_json::from_reader(file).unwrap()
    }
});

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

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ens {
    /// ENS registry address
    pub registry: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Explorer {
    pub name: String,
    pub url: String,
    pub standard: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bridge {
    pub url: String,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_value: String,
    pub chain: String,
    pub bridges: Option<Vec<Bridge>>,
}

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RedFlagItem {
    ReusedChainId,
}

/// EIP-155 Chain Data.
///
/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainData {
    /// Name of the network.
    pub name: String,

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

    pub chain_id: u64,

    pub network_id: u64,

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
    pub red_flags: Option<Vec<RedFlagItem>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mut items = 0;
        let mut sz = 0;
        for chain in (*CHAIN_DATA).clone().into_iter() {
            items += 1;
            sz += std::mem::size_of_val(&chain);
        }
        println!("ITEMS: {} items", items);
        println!("SIZE:  {} bytes", sz);
        println!("SIZE1: {} bytes", sz / items);
    }
}
