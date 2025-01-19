use anchor_lang::prelude::*;
use crate::{
    state::{
        Profile,
        Handle,
        Storage
    },
    constants::{
        MAX_HANDLE_LENGTH,
        MIN_HANDLE_LENGTH
    },
    errors::OSPError,
    event::ProfileCreated
};
use crate::state::FollowCondition;

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct InitProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"profile", user.key().as_ref()],
        bump,
        space = 8 + Profile::INIT_SPACE
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        init,
        payer = user,
        seeds = [b"handle", handle.as_bytes().as_ref()],
        bump,
        space = 8 + Handle::INIT_SPACE
    )]
    pub handle_account: Account<'info, Handle>,
    #[account(
        mut,
        seeds = [b"storage"],
        bump
    )]
    pub storage: Account<'info, Storage>,
    pub system_program: Program<'info, System>
}

impl<'info> InitProfile<'info> {
    pub fn validate_handle(&mut self, handle: String) -> Result<()> {
        require!(
            MIN_HANDLE_LENGTH <= handle.len() && handle.len() <= MAX_HANDLE_LENGTH,
            OSPError::InvalidHandle
        );
        for element in handle.bytes() {
            require!(
                element.is_ascii_digit() || element.is_ascii_lowercase() || element.eq(&('_' as u8)),
                OSPError::InvalidHandle
            );
        }
        Ok(())
    }

    pub fn init_profile(&mut self, handle: String, follow_condition: Option<FollowCondition>) -> Result<()> {
        self.storage.profile_counter += 1;
        self.profile.set_inner(Profile {
            id: self.storage.profile_counter,
            handle,
            followers: 0,
            following: 0,
            content_counter: 0,
            follow_condition
        });
        Ok(())
    }

    pub fn emit_init_profile(&mut self, ctx: Vec<u8>) -> Result<()> {
        emit!(ProfileCreated {
            id: self.profile.id,
            handle: self.profile.handle.clone(),
            user: self.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
            ctx
        });
        Ok(())
    }
}