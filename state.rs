use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub total_staked: u64,
}

#[account]
pub struct UserStake {
    pub owner: Pubkey,
    pub amount: u64,
}
