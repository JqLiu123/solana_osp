use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to,
        Mint,
        MintTo,
        Token,
        TokenAccount
    }
};

use crate::{
    errors::OSPError,
    state::{
        Community,
        Profile
    }, Joined
};
use crate::state::{JoinTicket};
use crate::state::PermissionType::CommunityNullAccess;

#[derive(Accounts)]
pub struct JoinCommunity<'info> {
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
        init,
        payer = user,
        seeds = [b"join", community.key().as_ref(), user.key().as_ref()],
        bump,
        space = 8 + JoinTicket::INIT_SPACE
    )]
    pub join_ticket: Account<'info, JoinTicket>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinCommunity<'info> {
    pub fn join_community(&mut self) -> Result<()> {
        self.join_ticket.set_inner(JoinTicket {
            level: 0,
            access_type: CommunityNullAccess,
            is_black: false,
            community_key: self.community.key(),
        });

        Ok(())
    }

    pub fn emit_join_event(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(Joined {
            joiner: self.user.key(),
            joiner_profile_id: self.profile.id,
            community_id: self.community.id,
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}