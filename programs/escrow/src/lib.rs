use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
};
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer};

use spl_token::instruction::AuthorityType;

declare_id!("8poGjoAGyUVK6Ups3yaUBxFxXUYXmhyBo92qxQRkyUtV");

#[program]
pub mod escrow {

    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";
    pub const PREFIX: &str = "auction_house";
    pub const FEE_PAYER: &str = "fee_payer";

    pub const ESCROW_ACCOUNT_SIZE: usize =
        32 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 2 + 1 + 1;

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
        let offer = &mut ctx.accounts.offer;
        // Transferring from initializer to taker
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        // require the offer has been accepted before exchange
        require!(offer.accepted, OTCDeskError::OfferNotAccepted);

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
        let escrow_house = &ctx.accounts.escrow_house;
        let system_program = &ctx.accounts.system_program;
        let escrow_house_treasury = &ctx.accounts.escrow_house_treasury;

        require!(
            *ctx.accounts.authority.key == escrow_house.initializer_key,
            OTCDeskError::AdminAuthorityMismatch
        );

        let escrow_house_key = escrow_house.key();

        let seeds = [
            PREFIX.as_bytes(),
            escrow_house_key.as_ref(),
            FEE_PAYER.as_bytes(),
            &[escrow_house.fee_payer_bump],
        ];

        invoke_signed(
            &system_instruction::transfer(
                &escrow_house_treasury.key(),
                &escrow_house.initializer_key.key(),
                amount,
            ),
            &[
                escrow_house.to_account_info(),
                escrow_house.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&seeds],
        )?;

        Ok(())
    }

    pub fn create_offer(ctx: Context<CreateOffer>, price: u32, size: u64) -> Result<u64> {
        let offer_owner = &ctx.accounts.owner;
        let system_program = &ctx.accounts.system_program;
        let offer_price = price;
        let escrow_house = &ctx.accounts.escrow_house;
        let offer = &mut ctx.accounts.offer;

        let escrow_payment_account = &ctx.accounts.escrow_house;
        let escrow_house_treasury = &ctx.accounts.escrow_house_treasury;

        let fees = ctx.accounts.escrow_house.seller_fee_basis_points;

        let total_fee = (fees as u128)
            .checked_mul(size as u128)
            .ok_or(OTCDeskError::NumericalOverflow)?
            .checked_div(10000)
            .ok_or(OTCDeskError::NumericalOverflow)? as u64;

        let offer_number: u16 = 0;

        // create an offer

        // setup an offer structure to save the details on each offer,
        offer.price = offer_price;
        offer.offer_index = offer_number;
        offer.sol_escrow = escrow_house.key();
        offer.seller = offer_owner.key();

        // transfer fees
        invoke(
            &system_instruction::transfer(
                &escrow_payment_account.key(),
                escrow_house_treasury.key,
                total_fee,
            ),
            &[
                escrow_payment_account.to_account_info(),
                escrow_house_treasury.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        Ok(total_fee)
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        let offer = &mut ctx.accounts.offer;

        //
        require!(
            *ctx.accounts.authority.key == *ctx.accounts.initializer.key,
            OTCDeskError::AdminAuthorityMismatch
        );

        // flip this to true, meaning offer has been accepted
        offer.accepted = true;

        Ok(())
    }
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
    pub offer: Account<'info, Offer>,
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
    /// Authority key for the Escrow .
    pub authority: Signer<'info>,

    /// Account that pays for fees if the marketplace executes sales.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// Escrow  instance fee account.
    #[account(mut, seeds=[PREFIX.as_bytes(), escrow_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=escrow_house.fee_payer_bump)]
    pub escrow_house_fee_account: UncheckedAccount<'info>,
    pub escrow_house_treasury: UncheckedAccount<'info>,
    pub escrow_payment_account: UncheckedAccount<'info>,

    /// Escrow  instance PDA account.
    #[account(mut, seeds=[PREFIX.as_bytes(), escrow_house.initializer_key.as_ref()], bump=escrow_house.bump, has_one=authority, has_one=fee_withdrawal_destination, has_one=escrow_house_fee_account)]
    pub escrow_house: Account<'info, EscrowAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    pub initializer: UncheckedAccount<'info>,
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: UncheckedAccount<'info>,
    pub authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub offer: Account<'info, Offer>,
}
#[derive(Accounts)]
pub struct CreateOffer<'info> {
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub escrow_house: Account<'info, EscrowAccount>,
    pub escrow_house_treasury: UncheckedAccount<'info>,
    pub fee_destination: UncheckedAccount<'info>,
    pub offer: Account<'info, Offer>,
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

    /// Escrow instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// Escrow instance PDA account.
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            escrow_house.initializer_key.as_ref(),
        ],
        bump=escrow_house.bump,
        has_one=authority,
        has_one=escrow_house_fee_account
    )]
    pub escrow_house: Box<Account<'info, EscrowAccount>>,

    /// Escrow  instance fee account.
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
    pub escrow_house_fee_account: Pubkey,
    pub escrow_house_fee_treasury: Pubkey,
    pub fee_withdrawal_destination: Pubkey,
    pub fee_payer: Pubkey,
    pub authority: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub seller_fee_basis_points: u16,
    pub bump: u8,
    pub fee_payer_bump: u8,
}

#[account]
pub struct Offer {
    pub seller: Pubkey,
    pub sol_escrow: Pubkey,
    pub listing_id: u128,
    pub amount: u32,
    pub price: u32,
    pub offer_index: u16,
    pub accepted: bool,
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
    #[msg("NumericalOverflow")]
    NumericalOverflow,

    #[msg("Admin Authority Mismatch")]
    AdminAuthorityMismatch,

    #[msg("Insufficient funds in escrow account to purchase.")]
    InsufficientFunds,

    #[msg("This Offer has not been accepted.")]
    OfferNotAccepted,
}
