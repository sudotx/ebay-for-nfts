use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::{
    _main::main_state::MainState,
    constants::{METAPLEX_PROGRAM_ID, SEED_MAIN_STATE, SEED_OFFER},
    error::OTCDeskError,
    events,
    offer::offer_state::OfferState,
};

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug)]
pub struct EditOfferInput {
    new_requested_token_amount: Option<u64>,
    new_min_requested_token_amount: Option<u64>,
}

pub fn edit_offer(ctx: Context<EditOffer>, input: EditOfferInput, symbol: String) -> Result<()> {
    // the offer in reference to
    let offer_state = &mut ctx.accounts.offer_state_account;

    let metaplex_pubkey = METAPLEX_PROGRAM_ID
        .parse::<Pubkey>()
        .expect("Failed to parse Metaplex Program Id");

    let mint = *ctx.accounts.mint.key;

    let seeds = &[
        "metadata".as_bytes(),
        metaplex_pubkey.as_ref(),
        mint.as_ref(),
    ];

    let (metadata_pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);

    if metadata_pda != *ctx.accounts.metadata.key {
        // return Err(ErrorCode::NoMatchMetadata.into());
    }

    // if symbol.as_bytes() != SYMBOL {
    //     // return Err(ErrorCode::NoMatchSymbol.into());
    // }

    // check if the offer is not currently active
    if !offer_state.is_active {
        return anchor_lang::err!(OTCDeskError::OfferNotActive);
    }

    if let Some(amount) = input.new_requested_token_amount.clone() {
        if amount == 0 {
            return anchor_lang::err!(OTCDeskError::ZeroRequestedAmount);
        }
        // set offer amount to the set value, if it is not zero
        offer_state.requested_amount = amount;
    }

    if let Some(amount) = input.new_min_requested_token_amount.clone() {
        if amount > offer_state.requested_amount {
            return anchor_lang::err!(OTCDeskError::TooHighAmount);
        }
        // if the requested amount exceeds the requested amount
        offer_state.min_requested_amount = amount;
    }

    emit!(events::OfferUpdated {
        offer_id: ctx.accounts.offer_state_account.key(),
        new_requested_token_amount: input.new_requested_token_amount,
        new_min_requested_token_amount: input.new_min_requested_token_amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct EditOffer<'info> {
    pub bidder: Signer<'info>,
    pub mint: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,

    #[account(
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,

    #[account(
        mut,
        seeds = [
            SEED_OFFER,
            offer_state_account.init_time.to_le_bytes().as_ref(),
            bidder.key().as_ref(),
            offer_state_account.offered_token.key().as_ref(),
            offer_state_account.requested_token.key().as_ref(),
        ],
        bump,
    )]
    pub offer_state_account: Account<'info, OfferState>,

    pub token_program: Program<'info, Token>,
}
