use {
    anchor_lang::{
        prelude::*,
    },
};

#[account]
pub struct OrderDetail {
    pub seller: Pubkey,
    pub token_id: Pubkey,
    pub price: u64,
}

#[account]
pub struct ListOrder {
    pub list_owner: Pubkey,
    pub data: Vec<Pubkey>,
}

