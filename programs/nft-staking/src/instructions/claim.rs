use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{VaultAccount, PoolAccount, REWARD_TOKEN_ADDRESS, OGAStakingError, REWARD_PER_DAY};

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
    pool_owner: Pubkey,
    vault_bump: u8,
    pool_bump: u8
)]
pub struct Claim<'info> {
    #[account(
        mut,
        seeds = [
          b"vault",
          mint_address.as_ref(),
          pool_account.key().as_ref(),
          user.key().as_ref()
        ],
        bump=vault_bump
    )]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(
        seeds = [
            b"pool",
            pool_owner.as_ref()
        ],
        bump=pool_bump
      )]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool_ata_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub user_ata_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
}

pub fn handler(
    ctx: Context<Claim>,
    _mint_address: Pubkey,
    pool_owner: Pubkey,
    _vault_bump: u8,
    pool_bump: u8
) -> Result<()> {

    let now_ts = Clock::get().unwrap().unix_timestamp;

    let vault_account = &mut ctx.accounts.vault_account;
    let recent_claimed_time = &vault_account.recent_claimed_time;

    let reward_amount_i64 = (now_ts - recent_claimed_time)/86400 * REWARD_PER_DAY;
    let reward_amount_u64: u64 = reward_amount_i64 as u64;
    
    let pool_ata_token_account = &ctx.accounts.pool_ata_token_account;
    let user_ata_token_account = &ctx.accounts.user_ata_token_account;

    if pool_ata_token_account.mint.to_string() != REWARD_TOKEN_ADDRESS.to_string() {
        return err!(OGAStakingError::UnknownError);
    }

    if user_ata_token_account.mint.to_string() != REWARD_TOKEN_ADDRESS.to_string() {
        return err!(OGAStakingError::UnknownError);
    }

    let pool_seeds = &[
      b"pool",
      pool_owner.as_ref(),
      &[pool_bump]
    ];

    let pool_signer = [&pool_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: user_ata_token_account.to_account_info(),
            to: user_ata_token_account.to_account_info(),
            authority: ctx.accounts.pool_account.to_account_info()
        },
        &pool_signer
      );
    
    token::transfer(cpi_ctx, reward_amount_u64)?;

    vault_account.recent_claimed_time = now_ts;

    Ok(())
}