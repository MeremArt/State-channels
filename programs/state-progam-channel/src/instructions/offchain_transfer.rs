use anchor_lang::prelude::*;
use crate::state::Channel;

// Off-chain transaction (signed message)
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OffChainTransferData {  
    pub amount: u64,
    pub nonce: u64,// ensures every transaction is processed only once
    pub signature: [u8; 64],
}

#[derive(Accounts)]
pub struct OffChainTransfer<'info> {
    #[account(
        mut,
        constraint = channel.nonce < transfer.nonce @ ErrorCode::InvalidNonce,
        constraint = channel.user_balance >= transfer.amount @ ErrorCode::InsufficientFunds
    )]
    pub channel: Account<'info, Channel>,
}

pub fn offchain_transfer(
    ctx: Context<OffChainTransfer>, 
    transfer: OffChainTransferData
) -> Result<()> {
    let channel = &mut ctx.accounts.channel;
    
    // Verify signature using Ed25519 program
    let message = [
        b"Transfer:",
        &transfer.amount.to_le_bytes(),
        b"Nonce:",
        &transfer.nonce.to_le_bytes(),
    ].concat();

    let ix = solana_program::ed25519_program::verify(
        &channel.user,
        &message,
        &transfer.signature,
    );
    
    solana_program::program::invoke(
        &ix,
        &[]  // No accounts needed for ed25519 verification
    )?;

    // Update balances after verification
    channel.user_balance = channel.user_balance
        .checked_sub(transfer.amount)
        .ok_or(ErrorCode::Overflow)?;
        
    channel.provider_balance = channel.provider_balance
        .checked_add(transfer.amount)
        .ok_or(ErrorCode::Overflow)?;

    channel.nonce = transfer.nonce;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid nonce")]
    InvalidNonce,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Arithmetic overflow")]
    Overflow,
}