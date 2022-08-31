use anchor_lang::prelude::*;

pub mod mint;
pub mod buy;
pub mod ordering;
pub mod state;
pub mod errors;

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
    ) -> Result<()> {
        buy::buy(
            ctx,
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
        token_id: Pubkey,
        new_price: u64
    ) -> Result<()> {
        ordering::update_order_detail(
            ctx,
            token_id,
            new_price,
        )
    }

    pub fn get_order_detail(
        ctx: Context<GetOrderDetail>,
        token_id: Pubkey,
    ) -> Result<()> {
        ordering::get_order_detail(
            ctx,
            token_id,
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