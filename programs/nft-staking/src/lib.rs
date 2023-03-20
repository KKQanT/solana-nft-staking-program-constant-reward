use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;
pub mod instructions;
pub mod constant;
pub mod errors;

pub use state::*;
pub use instructions::*;
pub use constant::*;
pub use errors::*;

#[program]
pub mod oga_staking {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>) -> Result<()> {
        instructions::init_pool::handler(ctx)
    }

    pub fn stake(
        ctx: Context<Stake>,
        mint_address: Pubkey,
        pool_bump: u8,
        metadata_bump: u8
    ) -> Result<()> {
        instructions::stake::handler(
            ctx, 
            mint_address, 
            pool_bump, 
            metadata_bump
        )
    }

    pub fn claim(
        ctx: Context<Claim>,
        mint_address: Pubkey,
        pool_owner: Pubkey,
        vault_bump: u8,
        pool_bump: u8
    ) -> Result<()> {
        instructions::claim::handler(
            ctx, 
            mint_address, 
            pool_owner, 
            vault_bump, 
            pool_bump
        )
    }

    pub fn unstake(
        ctx: Context<Unstake>,
        mint_address: Pubkey,
        pool_owner: Pubkey,
        vault_bump: u8,
        pool_bump: u8
    ) -> Result<()> {
        instructions::unstake::handler(
            ctx, 
            mint_address, 
            pool_owner, 
            vault_bump, 
            pool_bump
        )
    }

}