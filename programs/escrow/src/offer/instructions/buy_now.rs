use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    _main::main_state::MainState,
    constants::{METAPLEX_PROGRAM_ID, SEED_MAIN_STATE, SEED_OFFER},
    error::OTCDeskError,
    events,
    offer::offer_state::OfferState,
    utils::{transfer_token, transfer_token_from_bidder_state},
};

pub fn buy_now(ctx: Context<AcceptOffer>, amount: u64, symbol: String) -> Result<()> {
    let seller = ctx.accounts.seller.to_account_info();
    let seller_offered_token_ata = ctx.accounts.seller_offered_token_ata.to_account_info();
    let seller_requested_token_ata = ctx.accounts.seller_requested_token_ata.to_account_info();
    let bidder_requested_token_ata = ctx.accounts.bidder_requested_token_ata.to_account_info();
    let fee_receiver_ata = ctx.accounts.fee_receiver_ata.to_account_info();
    let main_state = &ctx.accounts.main_state_account;
    let offer_state = &mut ctx.accounts.offer_state_account;
    let offer_state_ata = ctx.accounts.offer_state_account_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

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

    if seller.key() == offer_state.bidder {
        return anchor_lang::err!(OTCDeskError::SelfOfferAccept);
    }

    if !offer_state.is_active {
        return anchor_lang::err!(OTCDeskError::OfferNotActive);
    }

    // check if too high
    if amount > offer_state.requested_amount {
        return anchor_lang::err!(OTCDeskError::TooHighAmount);
    }
    // check if too low
    if amount < offer_state.min_requested_amount {
        return anchor_lang::err!(OTCDeskError::TooLowAmount);
    }

    let partial_offered_amount =
        (offer_state.offered_amount as u128 * amount as u128) as u64 / offer_state.requested_amount; // potential bug (multiplied value can exceed u64)

    //NOTE: Transfering the fees
    let fees = (main_state.fee_rate * amount as f64) as u64;

    // check remaining balance
    if amount + fees > ctx.accounts.seller_requested_token_ata.amount {
        return anchor_lang::err!(OTCDeskError::NotEnoughToken);
    }

    // NOTE: Transfer the fees
    transfer_token(
        seller_requested_token_ata.to_account_info(),
        fee_receiver_ata,
        seller.to_account_info(),
        token_program.to_account_info(),
        fees,
    )
    .map_err(|_| OTCDeskError::NotEnoughToken)?;

    //NOTE: Transfering the requested token to bidder ata
    transfer_token(
        seller_requested_token_ata.to_account_info(),
        bidder_requested_token_ata,
        seller.to_account_info(),
        token_program.to_account_info(),
        1,
    )
    .map_err(|_| OTCDeskError::NotEnoughToken)?;

    //NOTE: Transfer token from program account to seller
    transfer_token_from_bidder_state(
        offer_state,
        offer_state_ata,
        seller_offered_token_ata,
        token_program,
        amount,
    )?;

    //NOTE: set the state
    offer_state.offered_amount -= partial_offered_amount;
    offer_state.requested_amount -= amount;

    emit!(events::OfferAccepted {
        offer_id: offer_state.key(),
        amount: amount,
    });

    if offer_state.min_requested_amount > offer_state.requested_amount {
        if offer_state.requested_amount == 0 {
            emit!(events::OfferCompleted {
                offer_id: offer_state.key()
            });
            offer_state.re_init();
        }
    } else {
        offer_state.min_requested_amount = offer_state.requested_amount;
    }

    Ok(())
}

// main state_account: represents the program
// seller_offered_token_ata: sellers token account
// seller_requested_token_ata: token account of the seller
// bidder_requested_token_ata: token account of the bidder
// offer_state_account: offer pda
// offer_state_account_ata: offer token
// offer_state_account
// offer_state_account_ata
// fee_receiver_ata

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    pub seller: Signer<'info>,
    pub mint: AccountInfo<'info>,
    pub metadata: AccountInfo<'info>,

    #[account(
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,

    ///CHECK:
    #[account(mut)]
    pub seller_offered_token_ata: Account<'info, TokenAccount>,

    ///CHECK:
    #[account(mut)]
    pub seller_requested_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = offer_state_account.requested_token,
        token::authority =  offer_state_account.bidder,
    )]
    pub bidder_requested_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            SEED_OFFER,
            offer_state_account.init_time.to_le_bytes().as_ref(),
            offer_state_account.bidder.as_ref(),
            offer_state_account.offered_token.key().as_ref(),
            offer_state_account.requested_token.key().as_ref(),
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
        token::mint = offer_state_account.requested_token,
        token::authority = main_state_account.fee_receiver,
    )]
    pub fee_receiver_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
