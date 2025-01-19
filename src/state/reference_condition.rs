use anchor_lang::prelude::*;

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, InitSpace, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum ReferenceCondition {
    OnlyFollowers,
    SameCommunity,
}