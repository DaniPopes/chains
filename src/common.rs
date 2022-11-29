use alloc::string::String;
use serde::{Deserialize, Serialize};

/// Schema: https://github.com/ethereum-lists/chains/blob/master/tools/schema/chainSchema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativeCurrency {
    /// Name of the native currency.
    pub name: String,
    /// Symbol of the native currency.
    pub symbol: String,
    /// Decimals of the native currency.
    pub decimals: u8,
}
