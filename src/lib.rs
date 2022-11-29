//! # chains

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use once_cell::sync::Lazy;

#[doc(hidden)]
mod common;

#[cfg(not(feature = "mini"))]
mod normal;
#[cfg(not(feature = "mini"))]
pub use normal::*;

#[cfg(feature = "mini")]
mod mini;
#[cfg(feature = "mini")]
pub use mini::*;

pub static CHAIN_DATA: Lazy<Vec<ChainData>> = Lazy::new(|| {
    #[cfg(not(feature = "zip"))]
    {
        #[cfg(not(feature = "mini"))]
        const JSON: &str = include_str!("../data/chains.json");
        #[cfg(feature = "mini")]
        const JSON: &str = include_str!("../data/chains_mini.json");

        serde_json::from_str(JSON).unwrap()
    }
    #[cfg(feature = "zip")]
    {
        #[cfg(not(feature = "mini"))]
        const ZIP: &[u8] = include_bytes!("../data/chains.zip");
        #[cfg(feature = "mini")]
        const ZIP: &[u8] = include_bytes!("../data/chains_mini.zip");

        let reader = std::io::Cursor::new(ZIP);
        let mut archive = zip::ZipArchive::new(reader).unwrap();
        let file = archive.by_index(0).unwrap();
        serde_json::from_reader(file).unwrap()
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let len = CHAIN_DATA.len();
        eprintln!("{len}");
        assert!(len >= 600);

        for (i, chain) in CHAIN_DATA.iter().enumerate() {
            eprintln!("{i}: {chain:?}\n");
            assert!(!chain.name.is_empty());
            assert!(chain.chain_id > 0);
            assert!(!chain.short_name.is_empty());
            // assert!(chain.network_id > 0);
            assert!(!chain.native_currency.name.is_empty());
            assert!(!chain.native_currency.symbol.is_empty());
            assert!(chain.native_currency.decimals > 0);
            // assert!(!chain.rpc.is_empty());
            // assert!(!chain.faucets.is_empty());
            // assert!(!chain.info_url.is_empty());
        }
    }
}
