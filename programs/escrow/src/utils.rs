use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3Builder, DelegateAuthorityItemV1, DelegateProgrammableConfigItemV1,
    DelegateTransferV1, RevokeProgrammableConfigItemV1, TransferV1, VerifyCollection,
};

use mpl_token_metadata::accounts::{Metadata, MetadataDelegateRecord, TokenRecord};
//
use mpl_token_metadata::{
    errors::MplTokenMetadataError, programs::MPL_TOKEN_METADATA_ID, MAX_NAME_LENGTH,
    MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
};

// use mpl_token_auth_rules::instruction::

use crate::{constants::SEED_OFFER, offer::offer_state::OfferState};

// make a function to verify the metadata of a collection

// This function describes transfer of a specific amount of tokens from one account to another

// - `from`: AccountInfo of the sender.
// - `to`: AccountInfo of the recipient.
// - `authority`: AccountInfo of the authority controlling the transfer.
// - `token_program`: AccountInfo of the token program.
// - `amount`: The amount of tokens to transfer.
pub fn transfer_token<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from,
        to,
        authority,
    };

    let cpi_context = CpiContext::new(token_program, cpi_accounts);
    token::transfer(cpi_context, amount)?;

    Ok(())
}

// This describes a function that transfers tokens from an account associated with the bidder's state to another

// Parameters:
// - `offer_state`: Mutable reference to the bidder's state account.
// - `offer_state_account_ata`: AccountInfo of the bidder's state account.
// - `receiver_ata`: AccountInfo of the recipient's associated token account.
// - `token_program`: AccountInfo of the token program.
// - `amount`: The amount of tokens to transfer.
pub fn transfer_token_from_bidder_state<'a>(
    offer_state: &mut Account<'a, OfferState>,
    offer_state_account_ata: AccountInfo<'a>,
    receiver_ata: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(
        &[
            SEED_OFFER,
            offer_state.init_time.to_le_bytes().as_ref(),
            offer_state.bidder.as_ref(),
            offer_state.offered_token.key().as_ref(),
            offer_state.requested_token.key().as_ref(),
        ],
        &crate::ID,
    );

    let cpi_accounts = Transfer {
        from: offer_state_account_ata,
        to: receiver_ata,
        authority: offer_state.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            token_program,
            cpi_accounts,
            &[&[
                SEED_OFFER,
                offer_state.init_time.to_le_bytes().as_ref(),
                offer_state.bidder.as_ref(),
                offer_state.offered_token.key().as_ref(),
                offer_state.requested_token.key().as_ref(),
                &[bump],
            ]],
        ),
        amount,
    )
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn transfer_nft(
    fro: TransferV1,
    from: UncheckedAccount,
    to: UncheckedAccount,
    authority: UncheckedAccount,
    metaplexMetadata: Metadata,
    rcrd: TokenRecord,
) -> Result<()> {
    // how do i transfer an NFT using the metaplex functions.

    // send from one account to another

    // let cpi_accounts = TransferV1{
    //     from,
    //     to,
    //     authority
    // }

    Ok(())
}
/// .

pub fn delegate_authority_of_nft(
    bro: DelegateAuthorityItemV1,
    frr: DelegateProgrammableConfigItemV1,
    delegateRecord: MetadataDelegateRecord,
    metadata: Metadata,
    tknRcrd: TokenRecord,
) {
    // let cpi_accounts = DelegateAuthorityItemV1{
    //     authority,
    //     authorization_rules,
    //     authorization_rules_program, delegate, delegate_record, master_edition, metadata, mint, payer, spl_token_program, system_program, sysvar_instructions, token, token_record
    // }
}
pub fn transfer_token_as_delegate(
    who: DelegateTransferV1,
    delegateRecord: MetadataDelegateRecord,
    pda: UncheckedAccount,
) {
}

pub fn revoke_delegate_authority_of_nft(
    to: RevokeProgrammableConfigItemV1,
    delegaterecord: MetadataDelegateRecord,
    frr: DelegateProgrammableConfigItemV1,
) {
    // an token authority revokes the previously issued delegate authority
    // it looks in the metadata delegate record, and deletes the record
}

// verify collection

pub fn get_collection_details<'a>(
    token: UncheckedAccount<'a>,
    verify_program: VerifyCollection,
    token_program: AccountInfo<'a>,
) {
    // retreive details of a collection from its public key

    // get details of a collection
}

// verify collection
pub fn verify_collection<'a>(
    token: UncheckedAccount<'a>,
    verify_program: VerifyCollection,
    token_program: AccountInfo<'a>,
    token_metadata: Metadata,
    expected_token_metadata: String,
) {
    // retreive details of a collection from its public key
    // add logic to derive metadata from the tokens public key

    // verify using the metaplex verified collections list

    let cpi_accounts = VerifyCollection {
        metadata: todo!(),
        collection_authority: todo!(),
        payer: todo!(),
        collection_mint: todo!(),
        collection: todo!(),
        collection_master_edition_account: todo!(),
        collection_authority_record: todo!(),
    };

    if token_metadata.name == expected_token_metadata {
        // do something
        // do some asserts
    }

    // token_metadata.collection_details()
    // verify_program.metadata()
}
