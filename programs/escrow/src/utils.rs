use crate::{EscrowAccount, OTCDeskError};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};

#[allow(clippy::too_many_arguments)]
pub fn pay_escrow_house_fees<'a>(
    escrow_house: &anchor_lang::prelude::Account<'a, EscrowAccount>,
    escrow_house_treasury: &AccountInfo<'a>,
    escrow_payment_account: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
    size: u64,
    is_native: bool,
) -> Result<u64> {
    let fees = escrow_house.seller_fee_basis_points;
    let total_fee = (fees as u128)
        .checked_mul(size as u128)
        .ok_or(OTCDeskError::NumericalOverflow)?
        .checked_div(10000)
        .ok_or(OTCDeskError::NumericalOverflow)? as u64;
    if !is_native {
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                escrow_payment_account.key,
                escrow_house_treasury.key,
                &escrow_house.key(),
                &[],
                total_fee,
            )?,
            &[
                escrow_payment_account.clone(),
                escrow_house_treasury.clone(),
                token_program.clone(),
                escrow_house.to_account_info(),
            ],
            &[signer_seeds],
        )?;
    } else {
        invoke_signed(
            &system_instruction::transfer(
                escrow_payment_account.key,
                escrow_house_treasury.key,
                total_fee,
            ),
            &[
                escrow_payment_account.clone(),
                escrow_house_treasury.clone(),
                system_program.clone(),
            ],
            &[signer_seeds],
        )?;
    }
    Ok(total_fee)
}
