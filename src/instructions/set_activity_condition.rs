use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, Mint, TokenAccount};
use crate::errors::OSPError;
use crate::state::{ActivityCondition, Community};

#[derive(Accounts)]
pub struct SetActivityCondition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub community: Box<Account<'info, Community>>,
    #[account(
        seeds = [b"community_mint", community.key().as_ref()],
        bump,
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub ata: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SetActivityCondition<'info> {
    pub fn set_activity_condition(&mut self, activity_condition: Option<ActivityCondition>) -> Result<()> {
        require_eq!(self.ata.amount, 1, OSPError::NoPermission);
        self.community.activity_condition = activity_condition;
        msg!("Activity conditions set to {:?}", self.community.activity_condition);
        Ok(())
    }
}