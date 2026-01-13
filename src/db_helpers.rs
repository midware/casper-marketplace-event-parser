use marketplace_indexer::events::{AuctionStarted, Bid, ListingBought, ListingCancelled, NewListing, NewOffer, OfferAccepted, OfferCancelled};

use crate::events::AuctionEnded;

fn substring_after_last_hyphen(input: &str) -> &str {
    if let Some(pos) = input.rfind('-') {
        &input[pos + 1..]
    } else {
        input
    }
}

pub async fn write_create_listing_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &NewListing,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "INSERT INTO node_casper_marketplace_event_create_listing (event_id, seller, contract_hash, timestamp, price, expiration_date, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&(event_id as i32), &substring_after_last_hyphen(&event.seller.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),  &(event.price.to_string()), &(event.expiration_date as i64), &event.token_id],
        )
        .await
}

pub async fn write_listing_bought_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &ListingBought,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "INSERT INTO node_casper_marketplace_event_listing_bought (event_id, seller, buyer, contract_hash, timestamp, price, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&(event_id as i32), &substring_after_last_hyphen(&event.seller.to_formatted_string()), &substring_after_last_hyphen(&event.buyer.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),  &(event.price.to_string()), &event.token_id],
        )
        .await
}

pub async fn write_listing_cancelled_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &ListingCancelled,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "INSERT INTO node_casper_marketplace_event_listing_cancelled (event_id, seller, contract_hash, timestamp, token_id) VALUES ($1, $2, $3, $4, $5)",
            &[&(event_id as i32), &substring_after_last_hyphen(&event.seller.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),  &event.token_id],
        )
        .await
}


pub async fn write_new_offer_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &NewOffer,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_new_offer (event_id, buyer, contract_hash, timestamp, price, expiration_date, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.buyer.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),  &(event.price.to_string()), &(event.expiration_date as i64), &event.token_id],
    )
        .await
}

pub async fn write_cancel_offer_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &OfferCancelled,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_offer_cancelled (event_id, buyer, contract_hash, timestamp,token_id) VALUES ($1, $2, $3, $4, $5)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.buyer.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),  &event.token_id],
    )
        .await
}

pub async fn write_accept_offer_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &OfferAccepted,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_offer_accepted (event_id,  buyer, seller, contract_hash, timestamp, price, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.buyer.to_formatted_string()), &substring_after_last_hyphen(&event.buyer.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),&(event.price.to_string()),  &event.token_id],
    )
        .await
}

//@TODO
pub async fn write_auction_started_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &AuctionStarted,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_auction_started (event_id,  seller, contract_hash, timestamp, expiration_date, price, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.seller.to_formatted_string()),  &event.contract_hash.to_string(), &(event.timestamp as i64), &(event.end_date as i64), &(event.starting_price.to_string()),  &event.token_id],
    )
        .await
}

//@TODO
pub async fn write_auction_bid_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &Bid,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_auction_bid (event_id,  bidder, seller, contract_hash, timestamp, new_end_timestamp, bid_price, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.bidder.to_formatted_string()), &substring_after_last_hyphen(&event.seller.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64), &(event.new_end_timestamp as i64),&(event.bid_price.to_string()),  &event.token_id],
    )
        .await
}


//@TODO
pub async fn write_auction_ended_event_to_db(
    db_client: &tokio_postgres::Client,
    event: &AuctionEnded,
    event_id: u32,
) -> Result<u64, tokio_postgres::Error> {
    db_client
    .execute(
        "INSERT INTO node_casper_marketplace_event_auction_ended (event_id, seller, winner, contract_hash, timestamp, ending_price, token_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&(event_id as i32), &substring_after_last_hyphen(&event.seller.to_formatted_string()), &substring_after_last_hyphen(&event.winner.to_formatted_string()), &event.contract_hash.to_string(), &(event.timestamp as i64),&(event.ending_price.to_string()),  &event.token_id],
    )
        .await
}


