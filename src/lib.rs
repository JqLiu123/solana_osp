mod errors;
mod constants;
mod state;
mod instructions;
mod event;

use anchor_lang::prelude::*;
use crate::instructions::*;
use crate::state::*;

declare_id!("Gzert5Absm5K8qRwVu7pdzUjymKSizb85vGWCue8yHon");

#[program]
pub mod open_social {
    use crate::constants::MAX_TAGS;
    use super::*;

    pub fn initialize_storage(ctx: Context<InitStorage>, seller_fee_basis_points: u16, community_price: u64) -> Result<()> {
        ctx.accounts.init_storage(seller_fee_basis_points, community_price)?;
        Ok(())
    }

    pub fn initialize_profile(ctx: Context<InitProfile>, handle: String, follow_condition: Option<FollowCondition>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.validate_handle(handle.clone())?;
        ctx.accounts.init_profile(handle.clone(), follow_condition)?;
        ctx.accounts.emit_init_profile(ix_ctx)?;
        Ok(())
    }

    pub fn set_follow_conditions(ctx: Context<SetFollowConditions>, follow_condition: Option<FollowCondition>) -> Result<()> {
        ctx.accounts.init_follow_conditions(follow_condition)
    }

    pub fn follow_profile(ctx: Context<FollowProfile>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.check_follow_conditions(ctx.remaining_accounts)?;
        ctx.accounts.follow_profile()?;
        ctx.accounts.emit_profile_followed(ix_ctx)
    }

    pub fn unfollow_profile(ctx: Context<UnfollowProfile>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.unfollow_profile()?;
        ctx.accounts.emit_profile_unfollowed(ix_ctx)
    }


    pub fn create_community(ctx: Context<InitCommunity>, handle: String, uri_community: String, activity_condition: Option<ActivityCondition>, tags: Vec<String>, ix_ctx: Vec<u8>) -> Result<()> {
        require!(tags.len() <= MAX_TAGS, errors::OSPError::TooManyTags);

        ctx.accounts.validate_handle(&handle)?;
        ctx.accounts.community_purchase()?;
        ctx.accounts.init_community(handle.clone(), activity_condition)?;
        ctx.accounts.mint_community_nft(uri_community, &ctx.bumps)?;
        ctx.accounts.emit_init_community_event(tags, handle, ix_ctx)
    }

    pub fn join_community(ctx: Context<JoinCommunity>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.join_community()?;
        ctx.accounts.emit_join_event(ix_ctx)
    }

    pub fn un_join_community(ctx: Context<UnJoinCommunity>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.un_join_community()?;
        ctx.accounts.emit_un_join_event(ix_ctx)
    }

    pub fn create_activity(ctx: Context<CreateActivity>, uri: String, reference_condition: Option<ReferenceCondition>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.create_activity(uri, &ctx.bumps, reference_condition, ctx.remaining_accounts)?;
        ctx.accounts.emit_activity_created(ix_ctx)
    }

    pub fn create_comment(ctx: Context<CreateComment>, uri: String, reference_condition: Option<ReferenceCondition>, ix_ctx: Vec<u8>) -> Result<()> {
        ctx.accounts.check_comment_conditions(ctx.remaining_accounts)?;
        ctx.accounts.create_comment(uri, &ctx.bumps, reference_condition)?;
        ctx.accounts.emit_comment_created(ix_ctx)
    }
}