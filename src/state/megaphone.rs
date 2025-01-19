use anchor_lang::prelude::*;
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug)]
pub enum Currency {
    SOL,
    USDC
}