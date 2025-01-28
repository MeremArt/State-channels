use anchor_lang::prelude::*;
use std::mem::size_of;

//This is the state channel account

#[account]
pub struct Channel {
    pub user:Pubkey ,
    pub provider:Pubkey,
    pub user_balance:u64,
    pub provider_balance:u64,
    pub nonce:u64,
    pub status: ChannelStatus,
    pub dispute_timeout: i64,
}

// this is the state struct for the channel

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ChannelStatus {
    Active,    // Channel is open for off-chain transactions
    Settling,  // Dispute initiated; waiting for timeout
    Closed,    // Channel closed after settlement
}