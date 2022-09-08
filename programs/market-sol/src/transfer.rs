use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

pub fn transfer(ctx: Context<TransferNFT>) -> Result<()> {
    msg!("Call transfer NFT");
    msg!("Vault Token Address: {}", &ctx.accounts.vault_token_account.key());
    let key = associated_token::get_associated_token_address(&ctx.accounts.vault_token_account.key(), &ctx.accounts.mint.key());
    if key.to_string().is_empty() {
        msg!("Create ATA for Vault");
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.owner_authority.to_account_info(),
                    associated_token: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;
    }
    msg!("Vault already exists ata with NFT");

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
    msg!("Vault Token Address: {}", &ctx.accounts.vault_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
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
    pub vault_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
