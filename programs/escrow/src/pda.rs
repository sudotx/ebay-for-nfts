use super::constants::*;
use crate::id;
use anchor_lang::prelude::Pubkey;

pub fn find_escrow_house_fee_pda(escrow_house_address: &Pubkey) -> (Pubkey, u8) {
    let escrow_fee_account_seeds = &[
        PREFIX.as_bytes(),
        escrow_house_address.as_ref(),
        FEE_PAYER.as_bytes(),
    ];
    Pubkey::find_program_address(escrow_fee_account_seeds, &id())
}
