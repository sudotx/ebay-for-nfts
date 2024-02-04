use crate::{_main::main_state::MainState, constants::SEED_MAIN_STATE, error::OTCDeskError};
use anchor_lang::prelude::*;

pub fn update_main_state_owner(
    ctx: Context<UpdateMainStateOwner>,
    new_owner: Pubkey,
) -> Result<()> {
    let state = &mut ctx.accounts.main_state_account;
    state.owner = new_owner;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMainStateOwner<'info> {
    #[account(
        mut,
        address = main_state_account.owner @ OTCDeskError::OnlyOwnerCanCall,
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds=[SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,

    pub system_program: Program<'info, System>,
}
