mod _main;
mod constants;
mod error;
mod events;
mod offer;
mod utils;

use _main::*;
use offer::*;

use anchor_lang::prelude::*;

declare_id!("8poGjoAGyUVK6Ups3yaUBxFxXUYXmhyBo92qxQRkyUtV");

// Alice creates a listing
// Bob creates an offer for said listing at a specified offer price
// Alice accepts or declines that offer
// once offer is accepted, create a pda from the mint account(of listing), ata of Alice, ata of Bob
// exchange happens
// spl token is sent to Bobs ata
// escrow pda account is closed, tokens sent back to Alice

#[program]
pub mod escrow {
    use super::*;
    // initialize main state of the program
    // Sets the initial state of the program
    pub fn initialize_main_state(ctx: Context<InitMainState>, input: MainStateInput) -> Result<()> {
        _main::init_main_state(ctx, input)?;
        Ok(())
    }
    // updates main state
    pub fn update_main_state(ctx: Context<UpdateMainState>, input: MainStateInput) -> Result<()> {
        _main::update_main_state(ctx, input)?;
        Ok(())
    }
    // update main state owner to a new owner
    pub fn update_main_state_owner(
        ctx: Context<UpdateMainStateOwner>,
        new_owner: Pubkey,
    ) -> Result<()> {
        _main::update_main_state_owner(ctx, new_owner)?;
        Ok(())
    }
    // initialize an offer state
    pub fn init_offer_state(ctx: Context<InitOfferState>, init_time: i64) -> Result<()> {
        offer::init_offer_state(ctx, init_time)?;
        Ok(())
    }
    // create an offer
    pub fn create_offer(
        ctx: Context<CreateOffer>,
        offered_amount: u64,
        requested_amount: u64,
        min_requested_amount: u64,
    ) -> Result<()> {
        offer::create_offer(ctx, offered_amount, requested_amount, min_requested_amount)?;
        Ok(())
    }
    // edit an offer
    pub fn edit_offer(ctx: Context<EditOffer>, input: EditOfferInput) -> Result<()> {
        offer::edit_offer(ctx, input)?;
        Ok(())
    }
    // accept an offer
    pub fn accept_offer(ctx: Context<AcceptOffer>, requested_amount: u64) -> Result<()> {
        offer::accept_offer(ctx, requested_amount)?;
        Ok(())
    }
    // close an offer
    pub fn close_offer(ctx: Context<CloseOffer>) -> Result<()> {
        offer::close_offer(ctx)?;
        Ok(())
    }
}
