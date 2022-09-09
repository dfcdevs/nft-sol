use {
    anchor_lang::{
        prelude::*,
        solana_program::program_pack::Pack,
    },
    anchor_spl::{
        associated_token,
        token,
        token::spl_token,
    },
    crate::errors::ErrorCode,
};

pub fn transfer(ctx: Context<TransferNFT>) -> Result<()> {
    msg!("Call transfer NFT");
    msg!("Receiver Token Address: {}", &ctx.accounts.receiver_token_account.key());
    if *ctx.accounts.receiver_token_account.owner == spl_token::id() {
        let ata_state = spl_token::state::Account::unpack(&ctx.accounts.receiver_token_account.data.borrow())?;
        // This can only happen if the owner transfered ownership to someone else but let's check anyway
        if ata_state.owner != ctx.accounts.receiver_authority.key() {
            return Err(error!(ErrorCode::InvalidInput));
        }
        msg!("ATA already exists");
    }
    else {
        msg!("Create ATA for Receiver");
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.owner_authority.to_account_info(),
                    associated_token: ctx.accounts.receiver_token_account.to_account_info(),
                    authority: ctx.accounts.receiver_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;
    }
    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
    msg!("Receiver Token Address: {}", &ctx.accounts.receiver_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.owner_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully");

    Ok(())
}

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub receiver_token_account: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub receiver_authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
