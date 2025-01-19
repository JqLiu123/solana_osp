use anchor_lang::prelude::*;
use crate::constants::MAX_HANDLE_LENGTH;
use crate::state::follow_condition::FollowCondition;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub id: u32,
    #[max_len(MAX_HANDLE_LENGTH)]
    pub handle: String,
    pub followers: u32,
    pub following: u32,
    pub content_counter: u32,
    pub follow_condition: Option<FollowCondition>
}

#[account]
#[derive(InitSpace)]
pub struct Handle {}

#[account]
#[derive(InitSpace)]
pub struct FollowTicket {}