use {
    anchor_lang::{
        prelude::*,
    },
};

#[error_code]
pub enum ErrorCode {
    #[msg("Wallet not onwer this order")]
    NotOwner,
    #[msg("Token already listing")]
    AlreadyListing,
    #[msg("Sale price must greater zero")]
    PriceMustGreaterZero,
    #[msg("Invalid input")]
    InvalidInput,
}