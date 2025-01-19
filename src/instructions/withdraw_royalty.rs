use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Authority;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
    mut,
    address = authority.receiver,
    )]
    pub receiver: Signer<'info>,
    #[account(
    mut,
    seeds = [b"collection_authority"],
    bump
    )]
    pub authority: Account<'info, Authority>,
    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw_royalty(&mut self, bumps: &WithdrawBumps) -> Result<()> {
        let lamports = self.authority.get_lamports();
        require_gt!(lamports, self.authority.rent);

        let dif = lamports - self.authority.rent;
        self.authority.sub_lamports(dif)?;
        self.receiver.add_lamports(dif)?;
        Ok(())
    }
}