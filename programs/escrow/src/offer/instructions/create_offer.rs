use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    _main::main_state::MainState,
    constants::{METAPLEX_PROGRAM_ID, SEED_MAIN_STATE, SEED_OFFER},
    error::OTCDeskError,
    events,
    offer::offer_state::OfferState,
    utils::transfer_token,
};

pub fn create_offer(
    ctx: Context<CreateOffer>,
    offered_amount: u64,
    requested_amount: u64,
    min_requested_amount: u64,
) -> Result<()> {
    let bidder = ctx.accounts.bidder.to_account_info();
    let bidder_ata = ctx.accounts.bidder_ata.to_account_info();
    let fee_receiver_ata = ctx.accounts.fee_receiver_ata.to_account_info();
    let main_state = &ctx.accounts.main_state_account;
    let offer_state = &mut ctx.accounts.offer_state_account;
    let offer_state_ata = ctx.accounts.offer_state_account_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let fees = (main_state.fee_rate * offered_amount as f64) as u64;

    if offer_state.is_active {
        return anchor_lang::err!(OTCDeskError::OfferAlreadyCreated);
    }

    if min_requested_amount > requested_amount {
        return anchor_lang::err!(OTCDeskError::TooHighAmount);
    }

    // the offered amount and fees must be higher than the bidders token balance
    if offered_amount + fees > ctx.accounts.bidder_ata.amount {
        return anchor_lang::err!(OTCDeskError::NotEnoughToken);
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

    //NOTE: seting state
    offer_state.offered_amount = offered_amount;
    offer_state.requested_amount = requested_amount;
    offer_state.min_requested_amount = min_requested_amount;
    offer_state.is_active = true;

    //NOTE: Transfering the fees
    transfer_token(
        bidder_ata.to_account_info(),
        fee_receiver_ata,
        bidder.to_account_info(),
        token_program.to_account_info(),
        fees,
    )?;

    //NOTE: Transfering the token to program account
    transfer_token(
        bidder_ata.to_account_info(),
        offer_state_ata,
        bidder.to_account_info(),
        token_program.to_account_info(),
        offered_amount,
    )?;

    emit!(events::OfferCreated {
        offer_id: offer_state.key(),
        bidder: ctx.accounts.bidder.key(),
        offered_token: offer_state.offered_token,
        requested_token: offer_state.requested_token,
        offered_amount,
        requested_amount,
        min_requested_amount,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    pub bidder: Signer<'info>,
    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,

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
        token::mint = offer_state_account.offered_token,
        token::authority = bidder
    )]
    pub bidder_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            SEED_OFFER,
            offer_state_account.init_time.to_le_bytes().as_ref(),
            bidder.key().as_ref(),
            offer_state_account.offered_token.key().as_ref(),offer_state_account.requested_token.key().as_ref()
        ],
        bump,
    )]
    pub offer_state_account: Account<'info, OfferState>,

    #[account(
        mut,
        token::mint = offer_state_account.offered_token,
        token::authority = offer_state_account,
    )]
    pub offer_state_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = offer_state_account.offered_token,
        token::authority = main_state_account.fee_receiver,
    )]
    pub fee_receiver_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
