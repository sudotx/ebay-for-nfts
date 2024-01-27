pub const PREFIX: &str = "auction_house";
pub const FEE_PAYER: &str = "fee_payer";
pub const TREASURY: &str = "treasury";
pub const SIGNER: &str = "signer";
pub const TRADE_STATE_SIZE: usize = 1;
pub const MAX_NUM_SCOPES: usize = 7;
pub const FEE: u64 = 100_000;

pub const ESCROW_HOUSE_SIZE: usize = 8 +                   // key
32 +                                                        // fee Payer
32 +                                                        // fee withdrawal destination
32 +                                                        // authority
32 +                                                        // creator
1 +                                                         // bump
1 +                                                         // fee_payer_bump
2 +                                                         // seller fee basis points
1 +                                                         // requires sign off
8 +                                                         // escrow payment bump
MAX_NUM_SCOPES +                                            // Array of AuthorityScope bools
172                                                         // padding
;

pub const LISTING_SIZE: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8;

pub const OFFER_SIZE: usize = 32 + 8 + 8 + 8 + 8 + 8;

pub const ESCROW_ACCOUNT_SIZE: usize = 32 + 32 + 32 + 8 + 8;
