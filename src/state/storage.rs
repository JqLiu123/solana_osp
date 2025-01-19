use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Storage {
    pub profile_counter: u32,
    pub community_counter: u32,
    pub megaphone_counter: u32,
    pub seller_fee_basis_points: u16,
    pub community_price: u64
}