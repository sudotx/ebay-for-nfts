mod constants;
mod pda;
mod utils;
use crate::{constants::*, utils::*};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
    AnchorDeserialize, AnchorSerialize,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer},
};
use spl_token::instruction::revoke;
use spl_token::instruction::AuthorityType;

use mpl_token_metadata::*;

use mpl_token_auth_rules::*;

declare_id!("8poGjoAGyUVK6Ups3yaUBxFxXUYXmhyBo92qxQRkyUtV");

#[program]
pub mod escrow {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    // initialize escrow
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;
        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;
        Ok(())
    }

    // cancel escrow
    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        token::set_authority(
            ctx.accounts
                .into_set_authority_context()
                .with_signer(&[&seeds[..]]),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.escrow_account.initializer_key),
        )?;

        Ok(())
    }

    // exchange
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        // Transferring from initializer to taker
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.taker_amount,
        )?;

        token::set_authority(
            ctx.accounts
                .into_set_authority_context()
                .with_signer(&[&seeds[..]]),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.escrow_account.initializer_key),
        )?;

        Ok(())
    }

    pub fn withdraw_from_fee_account(ctx: Context<WithdrawFromFee>, amount: u64) -> Result<()> {
        let escrow_house_fee_account = &ctx.accounts.escrow_house_fee_account;
        let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
        let escrow_house = &ctx.accounts.escrow_house;
        let system_program = &ctx.accounts.system_program;

        let escrow_house_key = escrow_house.key();

        let seeds = [
            PREFIX.as_bytes(),
            escrow_house_key.as_ref(),
            FEE_PAYER.as_bytes(),
            &[escrow_house.fee_payer_bump],
        ];

        invoke_signed(
            &system_instruction::transfer(
                &escrow_house_fee_account.key(),
                &fee_withdrawal_destination.key(),
                amount,
            ),
            &[
                escrow_house_fee_account.to_account_info(),
                fee_withdrawal_destination.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&seeds],
        )?;

        Ok(())
    }

    // enable seller list an asset they want to sell
    // set token authority to pda

    // get escrow signer keys

    // get_fee_payer

    // is_native
    // create_program_token_account_if_not_present

    // if asset os not native
    // assert_is_ata
    // transfer tokens to escrow payment account

    // else
    // add rent shorfall
    // and transfer lamports to the escrow payment account

    // get escrow signer keys

    // get_fee_payer

    // is_native
    // create_program_token_account_if_not_present

    // if asset os not native
    // assert_is_ata
    // transfer tokens to escrow payment account

    // else
    // add rent shorfall
    // and transfer lamports to the escrow payment account
    pub fn create_listing(ctx: Context<CreateListing>, price: u32, quantity: u32) -> Result<()> {
        let listing_authority = &ctx.accounts.authority;
        let escrow_house = &ctx.accounts.escrow_house;
        let fee_mint = &ctx.accounts.fee_mint;
        let fee_destination = &ctx.accounts.fee_withdrawal_destination;
        let listing_mint = &ctx.accounts.listing_mint;
        let fee_payer = &ctx.accounts.payer;
        let token_program = &ctx.accounts.token_program;
        let listing_price = price;
        let list_quantity = quantity;

        Ok(())
    }

    // Transferring from initializer to taker

    // require stuff
    // get Escrow house key
    // and seeds

    // get fee payer,

    // get fee payer

    // check if treasury token is the native mint

    // create_program_token_account_if_not_present

    // if treasury token is native,
    // 1. wallet key == payment account key
    // 2. if escrow account has more lamports than the buyer price
    // add it with the minimum rent for the escrow payment account
    // 3. transfer lamports to the escrow payment account

    // else, it is priced in spl token
    // get price of the spl token via oracle // in case of buying with another nft
    // init spl token state and wtf
    // check if the spl token price os more than the buyer price
    // transfer to the escrow payment account

    // check if buyer trade state exists
    // then create a new account for this
    // allow buyers to create offers
    // for listed assets

    // get escrow signer keys

    // get_fee_payer

    // is_native
    // create_program_token_account_if_not_present

    // if asset os not native
    // assert_is_ata
    // transfer tokens to escrow payment account

    // else
    // add rent shorfall
    // and transfer lamports to the escrow payment account
    pub fn create_offer(ctx: Context<CreateOffer>, price: u32, quantity: u32) -> Result<()> {
        let listing_mint = &ctx.accounts.listing_mint;
        let offer_owner = &ctx.accounts.owner;
        let offer_seller = &ctx.accounts.seller;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let offer_price = price;
        let offer_quantity = quantity;

        Ok(())
    }
    //
    // if !wallet.to_account_info().is_signer
    //     && (buyer_price == 0
    //         || free_seller_trade_state.data_is_empty()
    //         || !authority.to_account_info().is_signer
    //         || !escrow_house.can_change_sale_price)
    // {
    //     return Err(EscrowHouseError::SaleRequiresSigner.into());
    // }

    // get_fee_payer
    // assert_is_ata
    // assert_metadata_valid

    // require token_size > token_account.amount

    // wallet is signer
    // set token delegate
    //

    // create a new account to represent the seller trade state

    // allow seller to accept offers
    // and initiate the `trade`

    // get escrow signer keys

    // get_fee_payer

    // is_native
    // create_program_token_account_if_not_present

    // if asset os not native
    // assert_is_ata
    // transfer tokens to escrow payment account

    // else
    // add rent shorfall
    // and transfer lamports to the escrow payment account
    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        let offer_authority = &ctx.accounts.authority;
        let listing_mint = &ctx.accounts.mint;
        let offer_owner = &ctx.accounts.owner;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        Ok(())
    }
    // enable buyers and sellers,
    // to negotiote by creating counteroffer

    // get escrow signer keys

    // get_fee_payer

    // is_native
    // create_program_token_account_if_not_present

    // if asset os not native
    // assert_is_ata
    // transfer tokens to escrow payment account

    // else
    // add rent shorfall
    // and transfer lamports to the escrow payment account
    pub fn counter_offer(ctx: Context<CounterOffer>) -> Result<()> {
        let listing_mint = &ctx.accounts.mint;
        let owner = &ctx.accounts.owner;
        let seller = &ctx.accounts.seller;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        Ok(())
    }
}

pub enum EscrowStatus {
    Open,
    Canceled,
    Completed,
}

#[derive(Accounts)]
#[instruction(initializer_amount: u64)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = initializer, space = 8 + ESCROW_ACCOUNT_SIZE)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(signer)]
    /// CHECK:
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub initializer_main_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *pda_deposit_token_account.to_account_info().key,
        constraint = escrow_account.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer_main_account.key,
        close = initializer_main_account
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    /// CHECK:
    pub pda_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

// cancel escrow
#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    /// CHECK:
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub pda_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *pda_deposit_token_account.to_account_info().key,
        close = initializer
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
}

// withdraw from fee
#[derive(Accounts)]
pub struct WithdrawFromFee<'info> {
    /// Authority key for the Escrow House.
    pub authority: Signer<'info>,

    /// Account that pays for fees if the marketplace executes sales.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// Escrow House instance fee account.
    #[account(mut, seeds=[PREFIX.as_bytes(), escrow_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=escrow_house.fee_payer_bump)]
    pub escrow_house_fee_account: UncheckedAccount<'info>,

    /// Escrow House instance PDA account.
    #[account(mut, seeds=[PREFIX.as_bytes(), escrow_house.creator.as_ref()], bump=escrow_house.bump, has_one=authority, has_one=fee_withdrawal_destination, has_one=escrow_house_fee_account)]
    pub escrow_house: Account<'info, EscrowHouse>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
// close escrow account
#[derive(Accounts)]
pub struct CloseEscrowAccount<'info> {
    /// User wallet account.
    pub wallet: Signer<'info>,

    /// Buyer escrow payment account PDA.
    #[account(mut, seeds=[PREFIX.as_bytes(), escrow_house.key().as_ref(), wallet.key().as_ref()], bump=escrow_house.bump)]
    // change this bump to an appropriate one, escrow payment has its own bump
    pub escrow_payment_account: UncheckedAccount<'info>,

    /// Escrow House instance PDA account.
    #[account(seeds=[PREFIX.as_bytes(), escrow_house.creator.as_ref()], bump=escrow_house.bump)]
    pub escrow_house: Account<'info, EscrowHouse>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CounterOffer<'info> {
    pub seller: UncheckedAccount<'info>,
    pub owner: UncheckedAccount<'info>,
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: UncheckedAccount<'info>,
    pub authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateOffer<'info> {
    pub listing_mint: Account<'info, Mint>,
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateListing<'info> {
    /// fee mint account, either native SOL mint or a SPL token mint.
    pub fee_mint: Account<'info, Mint>,
    // mint account of the token, to be listed
    pub listing_mint: Account<'info, Mint>,
    /// Key paying SOL fees for setting up the Listing.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Authority key for the Listing.
    pub authority: UncheckedAccount<'info>,
    /// Account that pays for fees if the marketplace executes sales.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// Escrow House instance PDA account.
    pub escrow_house: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

// cancel
#[derive(Accounts)]
pub struct Cancel<'info> {
    /// User wallet account.
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// SPL token account containing the token of the sale to be canceled.
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// Token mint account of SPL token.
    pub token_mint: Box<Account<'info, Mint>>,

    /// Escrow House instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// Escrow House instance PDA account.
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            escrow_house.creator.as_ref(),
        ],
        bump=escrow_house.bump,
        has_one=authority,
        has_one=escrow_house_fee_account
    )]
    pub escrow_house: Box<Account<'info, EscrowHouse>>,

    /// Escrow House instance fee account.
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            escrow_house.key().as_ref(),
            FEE_PAYER.as_bytes()
        ],
        bump=escrow_house.fee_payer_bump
    )]
    pub escrow_house_fee_account: UncheckedAccount<'info>,

    /// Trade state PDA account representing the bid or ask to be canceled.
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// ACCOUNTS
#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub sol_escrow: Pubkey,
    pub listing_id: u128,
    pub listing_index: u16,
    pub price: u32,
    pub quantity: u32,
    /// Offer indexes start at 1. 0 means no offer accepted
    pub accepted_offer: u16,
    // Flip to true when done adding items
    pub finalized: bool,
}

#[account]
pub struct Offer {
    pub sol_escrow: Pubkey,
    pub listing_id: u128,
    pub amount: u32,
    pub price: u32,
    pub offer_index: u16,
    // Flip to true when done adding items.
    pub finalized: bool,
}

#[account]
pub struct EscrowHouse {
    pub escrow_house_fee_account: Pubkey,
    pub fee_payer: Pubkey,
    pub fee_withdrawal_destination: Pubkey,
    pub authority: Pubkey,
    pub creator: Pubkey,
    pub bump: u8,
    pub fee_payer_bump: u8,
    pub seller_fee_basis_points: u16,
    pub requires_sign_off: bool,
    pub escrow_payment_bump: u8,
    pub scopes: [bool; MAX_NUM_SCOPES],
}

impl<'info> From<&mut InitializeEscrow<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitializeEscrow<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            current_authority: accounts.initializer.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> CancelEscrow<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
            current_authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
            current_authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pda_deposit_token_account.to_account_info().clone(),
            to: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self
                .initializer_receive_token_account
                .to_account_info()
                .clone(),
            authority: self.taker.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[error_code]
pub enum OTCDeskError {
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,

    #[msg("InvalidMintAuthority")]
    InvalidMintAuthority,

    #[msg("UninitializedAccount")]
    UninitializedAccount,

    #[msg("IncorrectOwner")]
    IncorrectOwner,

    #[msg("PublicKeysShouldBeUnique")]
    PublicKeysShouldBeUnique,

    #[msg("StatementFalse")]
    StatementFalse,

    #[msg("NotRentExempt")]
    NotRentExempt,

    #[msg("NumericalOverflow")]
    NumericalOverflow,

    #[msg("Expected a sol account but got an spl token account instead")]
    ExpectedSolAccount,

    #[msg("Cannot exchange sol for sol")]
    CannotExchangeSOLForSol,

    #[msg("If paying with sol, sol wallet must be signer")]
    SOLWalletMustSign,

    #[msg("Cannot take this action without escrow house signing too")]
    CannotTakeThisActionWithoutEscrowHouseSignOff,

    #[msg("No payer present on this txn")]
    NoPayerPresent,

    #[msg("Derived key invalid")]
    DerivedKeyInvalid,

    #[msg("Metadata doesn't exist")]
    MetadataDoesntExist,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    #[msg("Both parties need to agree to this sale")]
    BothPartiesNeedToAgreeToSale,

    #[msg("Cannot match free sales unless the escrow house or seller signs off")]
    CannotMatchFreeSalesWithoutEscrowHouseOrSellerSignoff,

    #[msg("This sale requires a signer")]
    SaleRequiresSigner,

    #[msg("Old seller not initialized")]
    OldSellerNotInitialized,

    #[msg("Seller ata cannot have a delegate set")]
    SellerATACannotHaveDelegate,

    #[msg("Buyer ata cannot have a delegate set")]
    BuyerATACannotHaveDelegate,

    #[msg("No valid signer present")]
    NoValidSignerPresent,

    #[msg("BP must be less than or equal to 10000")]
    InvalidBasisPoints,

    #[msg("The trade state account does not exist")]
    TradeStateDoesntExist,

    #[msg("The trade state is not empty")]
    TradeStateIsNotEmpty,

    #[msg("The receipt is empty")]
    ReceiptIsEmpty,

    #[msg("The instruction does not match")]
    InstructionMismatch,

    #[msg("Invalid Admin for this Escrow House instance.")]
    InvalidAdmin,

    #[msg("The Admin does not have the correct scope for this action.")]
    MissingAdminScope,

    #[msg("Must use admin handler.")]
    MustUseAdminHandler,

    #[msg("No Admin program set.")]
    NoAdminProgramSet,

    #[msg("Too many scopes.")]
    TooManyScopes,

    #[msg("Escrow House not delegated.")]
    EscrowHouseNotDelegated,

    #[msg("Bump seed not in hash map.")]
    BumpSeedNotInHashMap,

    #[msg("The instruction would drain the escrow below rent exemption threshold")]
    EscrowUnderRentExemption,

    #[msg("Invalid seeds or Escrow House not delegated")]
    InvalidSeedsOrEscrowHouseNotDelegated,

    #[msg("The buyer trade state was unable to be initialized.")]
    BuyerTradeStateNotValid,

    #[msg("Amount of tokens available for purchase is less than the partial order amount.")]
    NotEnoughTokensAvailableForPurchase,

    #[msg("Escrow House already delegated.")]
    EscrowHouseAlreadyDelegated,

    #[msg("Admin Authority Mismatch")]
    AdminAuthorityMismatch,

    #[msg("Insufficient funds in escrow account to purchase.")]
    InsufficientFunds,

    #[msg("This sale requires exactly one signer: either the seller or the authority.")]
    SaleRequiresExactlyOneSigner,
}
