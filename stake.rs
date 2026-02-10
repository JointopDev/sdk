use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.total_staked += amount;
    Ok(())
}
