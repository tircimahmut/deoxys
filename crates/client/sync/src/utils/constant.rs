//! Deoxys constants.

pub mod starknet_core_address {
    pub const MAINNET: &str = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
    pub const GOERLI_TESTNET: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
    pub const GOERLI_INTEGRATION: &str = "0xd5c325D183C592C94998000C5e0EED9e6655c020";
    pub const SEPOLIA_TESTNET: &str = "0xE2Bb56ee936fd6433DC0F6e7e3b8365C906AA057";
    pub const SEPOLIA_INTEGRATION: &str = "0x4737c0c1B4D5b1A687B42610DdabEE781152359c";
}

pub const LOG_STATE_UPDTATE_TOPIC: &str = "0xd342ddf7a308dec111745b00315c14b7efb2bdae570a6856e088ed0c65a3576c";

pub mod bonsai_identifier {
    pub const CONTRACT: &[u8] = "0xcontract".as_bytes();
    pub const CLASS: &[u8] = "0xclass".as_bytes();
    pub const TRANSACTION: &[u8] = "0xtransaction".as_bytes();
    pub const EVENT: &[u8] = "0xevent".as_bytes();
}
