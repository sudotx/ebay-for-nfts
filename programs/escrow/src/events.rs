use anchor_lang::prelude::*;

#[event]
pub struct OfferCreated {
    pub offer_id: Pubkey,
    pub bidder: Pubkey,
    pub offered_token: Pubkey,
    pub requested_token: Pubkey,
    pub offered_amount: u64,
    pub requested_amount: u64,
    pub min_requested_amount: u64,
}

#[event]
pub struct OfferAccepted {
    pub offer_id: Pubkey,
    pub amount: u64,
}

#[event]
pub struct OfferUpdated {
    pub offer_id: Pubkey,
    pub new_requested_token_amount: Option<u64>,
    pub new_min_requested_token_amount: Option<u64>,
}

#[event]
pub struct OfferRevoked {
    pub offer_id: Pubkey,
}

#[event]
pub struct ListingCreated {
    pub listing_id: Pubkey,
}

#[event]
pub struct OfferCompleted {
    pub offer_id: Pubkey,
}
