use anchor_lang::prelude::*;
use crate::state::Storage;

#[derive(Accounts)]
pub struct InitStorage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
    init,
    payer = payer,
    seeds = [b"storage"],
    bump,
    space = 8 + Storage::INIT_SPACE
    )]
    pub storage: Account<'info, Storage>,
    pub system_program: Program<'info, System>
}

impl<'info> InitStorage<'info> {
    pub fn init_storage(&mut self, seller_fee_basis_points: u16, community_price: u64) -> Result<()> {
        self.storage.set_inner(Storage {
            profile_counter: 0,
            community_counter: 0,
            megaphone_counter: 0,
            seller_fee_basis_points,
            community_price
        });
        Ok(())
    }
}