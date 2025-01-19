use anchor_lang::prelude::*;

use crate::constants::{MAX_ACTIVITY_URL_LENGTH};

use super::ReferenceCondition;

#[account]
#[derive(InitSpace)]
pub struct Content {
    pub profile_id: u32,
    pub community_id: u32,
    pub content_id: u32,
    #[max_len(MAX_ACTIVITY_URL_LENGTH)]
    pub uri: String,
    pub reference_content_id: u32,
    pub reference_profile_id: u32,
    pub reference_condition: Option<ReferenceCondition>
}