use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Authority {
    pub receiver: Pubkey,
    pub rent: u64
}