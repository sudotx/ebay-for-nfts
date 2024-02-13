use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

use crate::{constants::SEED_OFFER, offer::offer_state::OfferState};

// This module provides functions for transferring tokens

// This function describes transfer of a specific amount of tokens from one account to another
// - `from`: AccountInfo of the sender.
// - `to`: AccountInfo of the recipient.
// - `authority`: AccountInfo of the authority controlling the transfer.
// - `token_program`: AccountInfo of the token program.
// - `amount`: The amount of tokens to transfer.
pub fn transfer_token<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from,
        to,
        authority,
    };

    let cpi_context = CpiContext::new(token_program, cpi_accounts);
    token::transfer(cpi_context, amount)?;

    Ok(())
}

// This describes a function that transfers tokens from an account associated with the bidder's state to another

// Parameters:
// - `offer_state`: Mutable reference to the bidder's state account.
// - `offer_state_account_ata`: AccountInfo of the bidder's state account.
// - `receiver_ata`: AccountInfo of the recipient's associated token account.
// - `token_program`: AccountInfo of the token program.
// - `amount`: The amount of tokens to transfer.
pub fn transfer_token_from_bidder_state<'a>(
    offer_state: &mut Account<'a, OfferState>,
    offer_state_account_ata: AccountInfo<'a>,
    receiver_ata: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(
        &[
            SEED_OFFER,
            offer_state.init_time.to_le_bytes().as_ref(),
            offer_state.bidder.as_ref(),
            offer_state.offered_token.key().as_ref(),
            offer_state.requested_token.key().as_ref(),
        ],
        &crate::ID,
    );

    let cpi_accounts = Transfer {
        from: offer_state_account_ata,
        to: receiver_ata,
        authority: offer_state.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            token_program,
            cpi_accounts,
            &[&[
                SEED_OFFER,
                offer_state.init_time.to_le_bytes().as_ref(),
                offer_state.bidder.as_ref(),
                offer_state.offered_token.key().as_ref(),
                offer_state.requested_token.key().as_ref(),
                &[bump],
            ]],
        ),
        amount,
    )
}
