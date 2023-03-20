use anchor_lang::prelude::*;

#[account]

pub struct VaultAccount {
    pub owner: Pubkey, //32
    pub pool: Pubkey, //32
    pub mint_address:  Pubkey, //32
    pub staked_time: i64, //8
    pub recent_claimed_time: i64, //8
}

impl VaultAccount {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8;
}