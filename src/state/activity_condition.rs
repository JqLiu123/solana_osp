use anchor_lang::prelude::*;
use crate::state::community::PermissionType;
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, InitSpace, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum ActivityCondition {
    AccessActivityCondition(PermissionType),
    HoldNFTActivityCondition(Pubkey),
}