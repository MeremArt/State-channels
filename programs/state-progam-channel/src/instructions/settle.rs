use anchor_lang::prelude::*;
use crate::state::Channel;

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub channel: Account<'info, Channel>,
    pub user: Signer<'info>,
    pub provider: Signer<'info>,
    pub system_program: Program<'info, System>,
}

