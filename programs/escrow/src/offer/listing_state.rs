use anchor_lang::prelude::*;

// bidder: current offer bidder
// seller: current offer seller
// offered token: token being offered
// requested token: token being requested in the offer
// requested_amount: amount requested for the offered token
// min_requested_amount: minimum amount of requested token in exchange for the offered token
// init_time: time of offer creation
// is_active: flag to indicate an offer is live or not

// // PDA ('offer', listing, offer_index) so it's easy to see offers on a listing
#[account]
#[derive(Default)]
pub struct CreateListingState {
    pub bidder: Pubkey,
    pub seller: Pubkey,
    pub offer_index: u16,
    pub offered_token: Pubkey, // listing
    pub requested_token: Pubkey,
    pub offered_amount: u64,
    pub requested_amount: u64,
    pub min_requested_amount: u64,
    pub init_time: i64,
    pub is_active: bool,
    pub num_items: u16,
    pub bump_seed: u8,
    // Both parties can start remove_from_escrow when num_items == num_items_escrowed for both accepted offer and listing
    pub num_items_escrowed: u16,
}

// impl ListingState {
//     pub const MAX_SIZE: usize = std::mem::size_of::<Self>();

//     // reinitialize the offer to a default state
//     pub fn re_init(&mut self) {
//         self.offered_amount = 0;
//         self.requested_amount = 0;
//         self.min_requested_amount = 0;
//         self.is_active = false;
//     }
// }
