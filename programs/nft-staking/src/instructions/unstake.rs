use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use crate::{VaultAccount, PoolAccount, EXPECTED_NFT_CREATOR_ADDRESS};

use crate::constant::METADATA_PROGRAM_ID;

use crate::errors::OGAStakingError;

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
    pool_owner: Pubkey,
    vault_bump: u8,
    pool_bump: u8
)]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [
          b"vault",
          mint_address.as_ref(),
          pool_account.key().as_ref(),
          user.key().as_ref()
        ],
        bump=vault_bump,
        close=user
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
    pub vault_nft_token_account: Account<'info, token::TokenAccount>, //Check in handler
    #[account(mut)]
    pub user_nft_token_account: Account<'info, token::TokenAccount>,
    ///CHECK: checked via instruction
    pub metadata_account: AccountInfo<'info>,
    ///CHECK : check via #[account(address = crate::address::METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    #[account(address = METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,

}

pub fn handler(
    ctx: Context<Unstake>,
    mint_address: Pubkey,
    _pool_owner: Pubkey,
    vault_bump: u8,
    _pool_bump: u8
) -> Result<()> {
    
    let user_nft_token_account = &ctx.accounts.user_nft_token_account;
    let user = &ctx.accounts.user;

    if user_nft_token_account.owner != user.key() {
        msg!("invalid owner");
        return  err!(OGAStakingError::UnknownError);
    }

    if user_nft_token_account.mint != mint_address {
        msg!("invalid mint");
        return  err!(OGAStakingError::UnknownError);
    }

    if user_nft_token_account.amount != 1 {
        msg!("invalid mint amount");
        return  err!(OGAStakingError::UnknownError);
    }

    let nft_metadata_account = &ctx.accounts.metadata_account;

    if nft_metadata_account.owner.key() != ctx.accounts.token_metadata_program.key() {
        msg!("invalid nft_metadata_account owner");
        return err!(OGAStakingError::UnknownError)
    };

    let metadata_seed = &[
        b"metadata",
        ctx.accounts.token_metadata_program.key.as_ref(),
        user_nft_token_account.mint.as_ref(),
    ];

    let (expected_metadata_key, _metadata_bump) = Pubkey::find_program_address(
        metadata_seed, 
        ctx.accounts.token_metadata_program.key
      );
    
    if  nft_metadata_account.key() != expected_metadata_key {
        msg!("invalid nft_metadata_account");
        return err!(OGAStakingError::UnknownError);
    }

    if  nft_metadata_account.data_is_empty() {
        msg!("data_is_empty");
        return  err!(OGAStakingError::UnknownError);
    }

    let nft_metadata: Metadata = Metadata::from_account_info(&nft_metadata_account)?;
    let nft_first_creator = &nft_metadata.data.creators.unwrap()[0];

    if !nft_first_creator.verified {
        msg!("not verified");
        return  err!(OGAStakingError::UnknownError);
    }
    
    if nft_first_creator.address.to_string() != EXPECTED_NFT_CREATOR_ADDRESS {
        msg!("invalid nft_first_creator");
        return  err!(OGAStakingError::UnknownError);
    }

    let vault_account = &mut ctx.accounts.vault_account;

    if user.key() != vault_account.owner {
        return  err!(OGAStakingError::UnknownError);
    }

    let expected_vault_token_account = associated_token::get_associated_token_address(
        &vault_account.key(), 
        &mint_address
      );

    if ctx.accounts.vault_nft_token_account.key() != expected_vault_token_account {
        msg!("invalid vault_nft_token_account");
        return err!(OGAStakingError::UnknownError);
    }

    if vault_account.mint_address != mint_address {
        msg!("invalid mint address in vault");
        return  err!(OGAStakingError::UnknownError);
    }

    let pool_account = &ctx.accounts.pool_account;
    let vault_nft_token_account = &mut ctx.accounts.vault_nft_token_account;
    let pool_account_key = pool_account.key();
    let user_key = user.key();

    let vault_seeds = &[
        b"vault",
        mint_address.as_ref(),
        pool_account_key.as_ref(),
        user_key.as_ref(),
        &[vault_bump]
    ];

    let vault_signer = [&vault_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        token::Transfer {
            from: vault_nft_token_account.to_account_info(),
            to: user_nft_token_account.to_account_info(),
            authority: vault_account.to_account_info()
        }, 
        &vault_signer
      );
    
    token::transfer(cpi_ctx, 1)?;

    let should_close_vault_token_account = {
        vault_nft_token_account.reload()?;
        vault_nft_token_account.amount == 0
      };
    
    if should_close_vault_token_account {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: vault_nft_token_account.to_account_info(),
                destination: user.to_account_info(),
                authority: vault_account.to_account_info()
            },
            &vault_signer
        );
        token::close_account(cpi_ctx)?;
      }
    
    Ok(())
    
}
