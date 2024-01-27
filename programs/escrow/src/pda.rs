use super::constants::*;
use crate::id;
use anchor_lang::prelude::Pubkey;

pub fn find_escrow_house_address(authority: &Pubkey, mint_address: &Pubkey) -> (Pubkey, u8) {
    let escrow_house_seeds = &[PREFIX.as_bytes(), authority.as_ref(), mint_address.as_ref()];
    Pubkey::find_program_address(escrow_house_seeds, &id())
}

pub fn find_escrow_house_fee_pda(escrow_house_address: &Pubkey) -> (Pubkey, u8) {
    let escrow_fee_account_seeds = &[
        PREFIX.as_bytes(),
        escrow_house_address.as_ref(),
        FEE_PAYER.as_bytes(),
    ];
    Pubkey::find_program_address(escrow_fee_account_seeds, &id())
}

pub fn find_escrow_house_buyer_pda(escrow_house_address: &Pubkey, wallet: &Pubkey) -> (Pubkey, u8) {
    let escrow_house_buyer_escrow_seeds = &[
        PREFIX.as_bytes(),
        escrow_house_address.as_ref(),
        wallet.as_ref(),
    ];
    Pubkey::find_program_address(escrow_house_buyer_escrow_seeds, &id())
}

pub fn find_escrow_payment_address_pda(escrow_house: &Pubkey, wallet: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[PREFIX.as_bytes(), escrow_house.as_ref(), wallet.as_ref()],
        &id(),
    )
}
