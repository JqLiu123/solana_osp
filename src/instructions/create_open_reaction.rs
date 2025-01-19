use anchor_lang::prelude::*;

use crate::{Content, OpenReaction, OpenReactionCreated, Profile};

#[derive(Accounts)]
pub struct CreateOpenReaction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,
    pub content: Account<'info, Content>,
}

impl<'info> CreateOpenReaction <'info> {
    pub fn serialize_open_reaction(&mut self, reaction: OpenReaction, timestamp: i64) -> Vec<u8> {
        let mut ctx: Vec<u8> = Vec::new();
        ctx.extend_from_slice(&self.profile.id.to_be_bytes());
        ctx.extend_from_slice(&self.content.community_id.to_le_bytes());
        ctx.extend_from_slice(&self.content.content_id.to_le_bytes());
        ctx.extend_from_slice(&self.content.profile_id.to_be_bytes());
        ctx.extend_from_slice(&timestamp.to_le_bytes());
        let reaction = reaction as u64;
        ctx.extend_from_slice(&reaction.to_le_bytes());
        ctx
    }
    pub fn emit_open_reaction_event(&mut self, reaction: OpenReaction, timestamp: i64) -> Result<()> {
        emit!(OpenReactionCreated {
            profile_id: self.profile.id,
            community_id: self.content.community_id,
            reference_content_id: self.content.content_id,
            reference_profile_id: self.content.profile_id,
            timestamp,
            reaction_value: reaction as u64,
            ctx: self.serialize_open_reaction(reaction, timestamp),
        });
        Ok(())
    }
}