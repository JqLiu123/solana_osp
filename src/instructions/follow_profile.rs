use anchor_lang::prelude::*;
use crate::{
    errors::OSPError, 
    state::{
        FollowCondition, 
        Profile
    },
    event::Followed
};
use crate::state::FollowTicket;

#[derive(Accounts)]
pub struct FollowProfile<'info> {
    #[account(mut)]
    pub follower: Signer<'info>,
    ///CHECK: followed account
    #[account()]
    pub followed: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"profile", follower.key().as_ref()],
        bump
    )]
    pub follower_profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [b"profile", followed.key().as_ref()],
        bump,
    )]
    pub followed_profile: Account<'info, Profile>,
    #[account(
        init,
        payer = follower,
        seeds = [b"follow", followed.key().as_ref(), follower.key().as_ref()],
        bump,
        space = 8 + FollowTicket::INIT_SPACE
    )]
    pub follow_ticket: Account<'info, FollowTicket>,
    pub system_program: Program<'info, System>,
}

impl<'info> FollowProfile<'info> {
    pub fn follow_profile(&mut self) -> Result<()> {

        self.follower_profile.following += 1;
        self.followed_profile.followers += 1;

        Ok(())
    }

    pub fn check_follow_conditions(&mut self, remaining_accounts: &[AccountInfo]) -> Result<()> {
        if let Some(condition) = &self.followed_profile.follow_condition {
            match condition {
                FollowCondition::IsFollowing { handle } => {
                    if remaining_accounts.len() != 3 {
                        return Err(OSPError::FollowConditionsNotMet.into());
                    }
                    let condition_followed_profile = &remaining_accounts[0];
                    let condition_followed_user = &remaining_accounts[1];
                    let condition_follow_ticket = &remaining_accounts[2];

                    let mut data = condition_followed_profile.try_borrow_mut_data()?;
                    let is_following_profile = Profile::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");
                    require_eq!(is_following_profile.handle.clone(), handle.clone(), OSPError::FollowConditionsNotMet);

                    let profile_address = Pubkey::find_program_address(&[b"profile", condition_followed_user.key().as_ref()], &crate::ID).0;
                    require_keys_eq!(condition_followed_profile.key(), profile_address, OSPError::FollowConditionsNotMet);

                    let follow_ticket_address = Pubkey::find_program_address(&[b"follow", condition_followed_user.key().as_ref(), &self.follower.key().as_ref()], &crate::ID).0;
                    data = condition_follow_ticket.try_borrow_mut_data()?;
                    FollowTicket::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");
                    require_keys_eq!(condition_follow_ticket.key(), follow_ticket_address, OSPError::FollowConditionsNotMet);
                },
                FollowCondition::MinimumFollowers(n) => {
                    require_gte!(self.follower_profile.followers, *n);
                },
            }
        }
        Ok(())
    }

    pub fn emit_profile_followed(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(Followed {
            follower: self.follower.key(),
            follower_profile_id: self.follower_profile.id,
            profile_id: self.followed_profile.id,
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}