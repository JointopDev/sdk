use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = signer, space = 8 + 40)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreatePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.authority = ctx.accounts.signer.key();
    pool.total_staked = 0;
    Ok(())
}
