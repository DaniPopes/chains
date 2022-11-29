//! # chains

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use once_cell::sync::Lazy;

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
        const S: &str = include_str!("../data/chains.json");
        #[cfg(feature = "mini")]
        const S: &str = include_str!("../data/chains_mini.json");

        serde_json::from_str(S).unwrap()
    }
    #[cfg(feature = "zip")]
    {
        #[cfg(not(feature = "mini"))]
        const Z: &[u8] = include_bytes!("../data/chains.zip");
        #[cfg(feature = "mini")]
        const Z: &[u8] = include_bytes!("../data/chains_mini.zip");

        let reader = std::io::Cursor::new(Z);
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
        assert!(CHAIN_DATA.len() > 500);
    }
}
