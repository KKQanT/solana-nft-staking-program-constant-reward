use anchor_lang::prelude::*;

use crate::state::PoolAccount;

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        seeds=[b"pool", pool_owner.key().as_ref()],
        bump,
        payer=pool_owner,
        space=PoolAccount::LEN
    )]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(mut)]
    pub pool_owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitPool>) -> Result<()> {
    msg!("initializing pool");
    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.owner = ctx.accounts.pool_owner.key();
    Ok(())
}