use casper_event_standard::Event;
use casper_types::{ContractHash, Key, U512};

#[derive(Event, Debug)]
pub struct NewListing {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
    pub expiration_date: u64
}

#[derive(Event, Debug)]
pub struct ListingBought {
    pub seller: Key,
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
}

#[derive(Event, Debug)]
pub struct ListingCancelled {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub timestamp: u64,
}

#[derive(Event, Debug)]
pub struct NewOffer {
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
    pub expiration_date: u64
}

#[derive(Event, Debug)]
pub struct OfferCancelled {
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub timestamp: u64,
}


#[derive(Event, Debug)]
pub struct OfferAccepted {
    pub buyer: Key,
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
}

#[derive(Event, Debug)]
pub struct AuctionStarted {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub starting_price: U512,
    pub timestamp: u64,
    pub end_date: u64
}

#[derive(Event, Debug)]
pub struct AuctionEnded {
    pub seller: Key,
    pub winner: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub ending_price: U512,
    pub timestamp: u64,
}



#[derive(Event, Debug)]
pub struct Bid {
    pub seller: Key,
    pub bidder: Key,
    pub contract_hash: ContractHash,
    pub bid_price: U512,
    pub token_id: String,
    pub timestamp: u64,
    pub new_end_timestamp: u64
}

#[derive(Event, Debug)]
pub struct RoyaltySet {
    pub recipient: Key,
    pub contract_hash: ContractHash,
    pub percentage: u64,
}