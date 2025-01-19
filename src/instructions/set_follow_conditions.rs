use anchor_lang::prelude::*;

use crate::state::{
    FollowCondition, 
    Profile
};

#[derive(Accounts)]
pub struct SetFollowConditions<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,
}

impl<'info> SetFollowConditions<'info> {
    pub fn init_follow_conditions(&mut self, follow_condition: Option<FollowCondition>) -> Result<()> {
        self.profile.follow_condition = follow_condition;
        msg!("Follow conditions set to {:?}", self.profile.follow_condition);
        Ok(())
    }
}