use anchor_lang::prelude::*;
use crate::{
    state::Profile,
    state::FollowTicket,
    Unfollowed
};

#[derive(Accounts)]
pub struct UnfollowProfile<'info> {
    #[account(mut)]
    pub follower: Signer<'info>,
    ///CHECK: followed account
    #[account()]
    pub followed: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"profile", follower.key().as_ref()],
        bump,
    )]
    pub follower_profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [b"profile", followed.key().as_ref()],
        bump,
    )]
    pub followed_profile: Account<'info, Profile>,
    #[account(
        mut,
        close = follower,
        seeds = [b"follow", followed.key().as_ref(), follower.key().as_ref()],
        bump,
    )]
    pub follow_ticket: Account<'info, FollowTicket>,

    pub system_program: Program<'info, System>,
}

impl<'info> UnfollowProfile<'info> {
    pub fn unfollow_profile(&mut self) -> Result<()> {
        self.follower_profile.following -= 1;
        self.followed_profile.followers -= 1;

        Ok(())
    }

    pub fn emit_profile_unfollowed(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(Unfollowed {
            follower: self.follower.key(),
            follower_profile_id: self.follower_profile.id,
            profile_id: self.followed_profile.id,
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}