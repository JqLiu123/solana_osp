use std::thread::sleep;
use std::time::Duration;
use anchor_lang::{
    prelude::*,
    system_program::{
        transfer,
        Transfer
    }
};
use anchor_spl::{
    token::{
        TokenAccount,
        Transfer as SplTransfer,
        transfer as spl_transfer
    }
};
use anchor_spl::token::Token;
use crate::{
    state::*,
    constants::USDC,
    errors::OSPError::ConditionalMegaphone
};
use crate::event::MegaphoneCreated;

#[derive(Accounts)]
pub struct CreateMegaphone<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    mut,
    seeds = [b"storage"],
    bump
    )]
    pub storage: Account<'info, Storage>,
    #[account(
    seeds = [b"collection_authority"],
    bump
    )]
    pub authority: Account<'info, Authority>,
    #[account(
    seeds = [b"profile", user.key().as_ref()],
    bump
    )]
    pub user_profile: Account<'info, Profile>,
    pub activity: Account<'info, Activity>,
    #[account(
    associated_token::mint = USDC,
    associated_token::authority = user
    )]
    pub user_ata: Option<Account<'info, TokenAccount>>,
    #[account(
    associated_token::mint = USDC,
    associated_token::authority = authority,
    )]
    pub authority_ata: Option<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateMegaphone<'info> {
    pub fn create_megaphone(&mut self, currency: Currency, amount: u64) -> Result<()> {
        require!(self.activity.reference_condition == None, ConditionalMegaphone);
        match currency {
            Currency::SOL => {
                transfer(
                    CpiContext::new(
                        self.system_program.to_account_info(),
                        Transfer {
                            from: self.user.to_account_info(),
                            to: self.authority.to_account_info()
                        }
                    ),
                    amount
                )
            }
            Currency::USDC => {
                spl_transfer(
                    CpiContext::new(
                        self.token_program.to_account_info(),
                        SplTransfer {
                            from: self.user_ata.as_ref().unwrap().to_account_info(),
                            to: self.authority_ata.as_ref().unwrap().to_account_info(),
                            authority: self.user.to_account_info()
                        }
                    ),
                    amount
                )
            }
        }
    }

    pub fn emit_create_megaphone_event(&mut self, tags: Vec<String>, start_time: i64, duration: u64, currency: Currency, amount: u64, ctx: Vec<u8>) -> Result<()> {
        self.storage.megaphone_counter += 1;
        emit!(MegaphoneCreated {
            profile_id: self.user_profile.id,
            reference_profile_id: self.activity.profile_id,
            reference_content_id: self.activity.content_id,
            tags,
            start_time,
            duration,
            currency,
            amount,
            ctx
        });
        Ok(())
    }
}