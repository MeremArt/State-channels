use anchor_lang::prelude::*;
use crate::state::{Channel, ChannelStatus};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 * size_of::<Channel>(),  seeds = [b"channel", user.key().as_ref(), provider.key().as_ref()], bump )]
    pub channel :Account<'info, Channel>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub provider: Signer<'info>, 
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx:Context<Initialize>,initial_balance:u64) -> Result<()>{
    let channel = &mut ctx.accounts.channel;
    channel.user = *ctx.accounts.user.key;
    channel.provider = *ctx.accounts.provider.key;
    channel.user_balance = initial_balance;
    channel.provider_balance = 0;
    channel.status = ChannelStatus::Active;
    channel.nonce = 0;
    Ok(())
}