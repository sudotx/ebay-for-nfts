use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    _main::main_state::MainState,
    constants::{METAPLEX_PROGRAM_ID, SEED_MAIN_STATE, SEED_OFFER},
    error::OTCDeskError,
    events,
    offer::offer_state::OfferState,
    utils::transfer_token_from_bidder_state,
};

pub fn close_offer(ctx: Context<CloseOffer>, symbol: String) -> Result<()> {
    let bidder_ata = ctx.accounts.bidder_ata.to_account_info();
    let offer_state = &mut ctx.accounts.offer_state_account;
    let offer_state_account_ata = ctx.accounts.offer_state_account_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

    if !offer_state.is_active {
        return anchor_lang::err!(OTCDeskError::OfferNotActive);
    }

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

    //NOTE: Let token amount transfer back to the bidder
    if offer_state.offered_amount > 0 {
        transfer_token_from_bidder_state(
            offer_state,
            offer_state_account_ata,
            bidder_ata,
            token_program,
            offer_state.offered_amount,
        )?;

        offer_state.re_init();
    }

    emit!(events::OfferRevoked {
        offer_id: offer_state.key(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CloseOffer<'info> {
    pub bidder: Signer<'info>,
    pub mint: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,
    ///CHECK:
    pub offered_token: AccountInfo<'info>,
    ///CHECK:
    pub requested_token: AccountInfo<'info>,
    #[account(
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,
    #[account(
        mut,
        seeds = [SEED_OFFER,offer_state_account.init_time.to_le_bytes().as_ref(),bidder.key().as_ref(),offer_state_account.offered_token.key().as_ref(),offer_state_account.requested_token.key().as_ref()],
        bump,
    )]
    pub offer_state_account: Account<'info, OfferState>,
    ///CHECK:
    #[account(
        mut,
        token::mint = offer_state_account.offered_token,
        token::authority = bidder,
    )]
    pub bidder_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = offer_state_account.offered_token,
        token::authority = offer_state_account,
    )]
    pub offer_state_account_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
