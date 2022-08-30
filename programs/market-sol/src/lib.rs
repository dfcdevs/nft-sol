use anchor_lang::prelude::*;

pub mod mint;
pub mod buy;
pub mod ordering;
pub mod state;
pub mod error;

use mint::*;
use buy::*;
use ordering::*;


declare_id!("ECpXc6PWu8ktZmJm4Qqit1zdeuSpTsAxzf3FmK9ULa7f");


#[program]
pub mod market_sol {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        mint::mint(
            ctx, 
            creator_key,
            uri,
            title,
        )
    }

    pub fn buy(
        ctx: Context<BuyNft>,
        price_order: u64
    ) -> Result<()> {
        buy::buy(
            ctx,
            price_order,
        )
    }

    pub fn create_order(
        ctx: Context<CreateOrder>,
        sale_lamports: u64
    ) -> Result<()> {
        ordering::create_order(
            ctx,
            sale_lamports,
        )
    }

    pub fn update_order_detail(
        ctx: Context<UpdateOrderDetail>,
        new_price: u64
    ) -> Result<()> {
        ordering::update_order_detail(
            ctx,
            new_price,
        )
    }

    pub fn setup_platform(
        ctx: Context<MarketPlatform>,
    ) -> Result<()> {
        ordering::setup_platform(
            ctx,
        )
    }

    pub fn clear_data(
        ctx: Context<ClearData>,
    ) -> Result<()> {
        ordering::clear_data(
            ctx,
        )
    }

}