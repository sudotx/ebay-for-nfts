use anchor_lang::prelude::*;

// main state of the program
#[account]
pub struct MainState {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_rate: f64,
}

impl MainState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct MainStateInput {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_rate: u64,
}

impl MainStateInput {
    // set the value of the owner,
    // fee receiver and current
    //  fee rate for being part of an offering
    pub fn set_value(&self, state: &mut Account<MainState>) {
        state.owner = self.owner;
        state.fee_receiver = self.fee_receiver;
        state.fee_rate = self.fee_rate as f64 / 1_000_000f64 / 100f64;
    }
}
