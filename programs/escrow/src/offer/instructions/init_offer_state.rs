use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{constants::SEED_OFFER, offer::offer_state::OfferState};

// initialize an offer
pub fn init_offer_state(ctx: Context<InitOfferState>, init_time: i64) -> Result<()> {
    let state = &mut ctx.accounts.offer_state_account;

    // set the state value to the current bidders
    state.bidder = ctx.accounts.bidder.key();
    state.offered_token = ctx.accounts.offered_token.key();
    state.requested_token = ctx.accounts.requested_token.key();
    state.init_time = init_time;

    Ok(())
}

#[derive(Accounts)]
#[instruction(init_time: i64)]
pub struct InitOfferState<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    pub offered_token: Account<'info, Mint>,
    pub requested_token: Account<'info, Mint>,

    #[account(init,payer= bidder,seeds=[SEED_OFFER, init_time.to_le_bytes().as_ref(),bidder.key().as_ref(), offered_token.key().as_ref(), requested_token.key().as_ref()],bump,space = 8 + OfferState::MAX_SIZE)]
    pub offer_state_account: Account<'info, OfferState>,

    #[account(
        init,
        payer = bidder,
        associated_token::mint = offered_token,
        associated_token::authority = offer_state_account,
    )]
    pub offer_state_account_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
