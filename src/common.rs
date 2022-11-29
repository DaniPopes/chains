use alloc::string::String;
use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::Chain;

/// A [Chain]'s native currency.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativeCurrency {
    /// Name of the native currency.
    pub name: String,
    /// Symbol of the native currency.
    pub symbol: String,
    /// Decimals of the native currency.
    pub decimals: u8,
}
