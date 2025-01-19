use anchor_lang::prelude::*;

use crate::{
    errors::OSPError,
    state::{
        Community,
        Profile
    }, Joined
};
use crate::event::UnJoined;
use crate::state::{JoinTicket};

#[derive(Accounts)]
pub struct UnJoinCommunity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"profile", user.key.as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [b"community", community.handle.as_str().as_bytes()],
        bump,
    )]
    pub community: Account<'info, Community>,
    #[account(
        mut,
        close = user,
        seeds = [b"join", community.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub join_ticket: Account<'info, JoinTicket>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnJoinCommunity<'info> {
    pub fn un_join_community(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn emit_un_join_event(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(UnJoined {
            user: self.user.key(),
            user_profile_id: self.profile.id,
            community_id: self.community.id,
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}