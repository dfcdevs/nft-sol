use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    crate::state::{
        OrderDetail,
        ListOrder,
    },
    crate::errors::ErrorCode,
};

pub fn buy(
    ctx: Context<BuyNft>,
) -> Result<()> {
    msg!("Call buy NFT {}", ctx.accounts.mint.key().to_string());
    let (pda, _) = Pubkey::find_program_address(&[ctx.accounts.mint.key().as_ref(), b"_"], ctx.program_id);
    msg!("pda: {}", pda);
    if pda != ctx.accounts.order_account.key() {
        return Err(error!(ErrorCode::InvalidInput))
    }

    msg!("Call buy nft {}, with price {} lamport...", ctx.accounts.mint.key().to_string(), ctx.accounts.order_account.price);
    msg!("Purchaser (sending lamports): {}", &ctx.accounts.buyer_authority.key());
    msg!("Seller (receiving lamports): {}", &ctx.accounts.seller.key());
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer_authority.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            }
        ),
        ctx.accounts.order_account.price
    )?;
    
    msg!("Lamports transferred successfully.");

    msg!("Creating buyer token account...");
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.owner_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");

    msg!("Remove list order item");
    let list_order_data = &mut &mut ctx.accounts.list_order_account.data;
    list_order_data.retain(|&key| key != ctx.accounts.mint.key());

    msg!("Sale completed successfully!");

    Ok(())
}


#[derive(Accounts)]
pub struct BuyNft<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub order_account: Account<'info, OrderDetail>,
    #[account(mut)]
    pub list_order_account: Account<'info, ListOrder>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}