use crate::{
    _main::main_state::{MainState, MainStateInput},
    constants::SEED_MAIN_STATE,
    error::OTCDeskError,
};
use anchor_lang::prelude::*;

pub fn update_main_state(ctx: Context<UpdateMainState>, input: MainStateInput) -> Result<()> {
    let state = &mut ctx.accounts.main_state_account;
    input.set_value(state);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMainState<'info> {
    #[account(mut, address = main_state_account.owner @ OTCDeskError::OnlyOwnerCanCall)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds=[SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateMainState<'info> {
    pub fn data_len(&self) -> usize {
        self.owner.data_len()
    }
}
