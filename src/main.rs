use std::time::Duration;
use std::env;

use casper_event_toolkit::error::ToolkitError;
use casper_types::bytesrepr::FromBytes;

use casper_event_toolkit::fetcher::{Fetcher, Schemas};
use casper_event_toolkit::metadata::CesMetadataRef;
use casper_event_toolkit::rpc::client::CasperClient;
use marketplace_indexer::events::{AuctionStarted, Bid, ListingBought, ListingCancelled, NewListing, NewOffer, OfferAccepted, OfferCancelled};
use std::convert::From;
use tokio_postgres::{NoTls};

use crate::db_helpers::{write_accept_offer_event_to_db, write_auction_bid_event_to_db, write_auction_ended_event_to_db, write_auction_started_event_to_db, write_cancel_offer_event_to_db, write_create_listing_event_to_db, write_listing_bought_event_to_db, write_listing_cancelled_event_to_db, write_new_offer_event_to_db};
use crate::events::AuctionEnded;

mod db_helpers;
mod events;

enum MyError {
    Toolkit(ToolkitError),
    Database(tokio_postgres::Error),
}

impl From<tokio_postgres::Error> for MyError {
    fn from(err: tokio_postgres::Error) -> MyError {
        MyError::Database(err)
    }
}

impl From<ToolkitError> for MyError {
    fn from(err: ToolkitError) -> MyError {
        MyError::Toolkit(err)
    }
}

fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_e) => "".to_string(),
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

 
    let host = get_env_var("DB_HOST"); 
    let user = get_env_var("DB_USER"); 
    let password = get_env_var("DB_PASSWORD"); 
    let dbname = get_env_var("DB_NAME");
    let port = get_env_var("DB_PORT");

    let marketplace_contract_hash = get_env_var("MARKETPLACE_CONTRACT_HASH");

    println!("Trying to connect to db...");
    let (db_client, connection) = match tokio_postgres::connect(
        &format!("host={} port={} user={} password={} dbname={}", host, port, user, password, dbname),
        NoTls,
    ).await {
        Ok((db_client, connection)) => (db_client, connection),
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });



    println!("*====================================*");
    println!("| Casper Event Toolkit               |");
    println!("*====================================*");
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let client = CasperClient::new("https://cspr-testnet.mystra.io:7778");

    println!("Fetching metadata of contract:");
    println!("{}", marketplace_contract_hash);
    std::thread::sleep(std::time::Duration::from_secs(1));

    let metadata = CesMetadataRef::fetch_metadata(
        &client,
        &marketplace_contract_hash
    )
    .await?;

    println!("-> events schema uref: {}", metadata.events_schema);
    println!("-> events count uref: {}", metadata.events_length);
    println!("-> events data uref: {}", metadata.events_data);
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let fetcher = Fetcher {
        client: CasperClient::new("https://cspr-testnet.mystra.io:7778"),
        ces_metadata: metadata,
    };

    println!("Extracting schema:");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let schemas: Schemas = fetcher.fetch_schema().await?;
    println!("-> {:?}", schemas);
    println!("\n");


    println!("Starting parser:");
    std::thread::sleep(std::time::Duration::from_secs(1));

    std::thread::sleep(std::time::Duration::from_secs(1));
    let _ = start_parsing(fetcher, schemas, &db_client).await;

    Ok(())
}

async fn start_parsing(
    fetcher: Fetcher,
    schemas: Schemas,
    client: &tokio_postgres::Client,
) -> Result<bool, MyError> {
    let mut event_id = 64;

    loop {
        println!("Fetching events count:");
        std::thread::sleep(std::time::Duration::from_secs(1));

        let num_events = fetcher.fetch_events_count().await?;
        println!("Events amount {}:", num_events);

        if num_events > event_id {
            println!("Fetching event {}:", event_id);
            std::thread::sleep(std::time::Duration::from_secs(1));

            let dynamic_event_result = fetcher.fetch_event(event_id.into(), &schemas).await;

            if let Err(e) = dynamic_event_result {
                eprintln!("Failed to fetch event: {}", e);
                std::process::exit(1);
            }
        
            let dynamic_event = dynamic_event_result.unwrap();
            println!("Parsing event {}:", event_id);
            std::thread::sleep(std::time::Duration::from_secs(1));

            match dynamic_event.name.as_str() {
                "NewListing" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = NewListing::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_create_listing_event_to_db(client, &parsed_further, event_id).await;
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                           
                        }
                    }
                }
                "ListingBought" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = ListingBought::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_listing_bought_event_to_db(client, &parsed_further, event_id).await;
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                           
                        }
                    }
                }
                "ListingCancelled" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = ListingCancelled::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_listing_cancelled_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "NewOffer" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = NewOffer::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_new_offer_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "OfferCancelled" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = OfferCancelled::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_cancel_offer_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "OfferAccepted" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = OfferAccepted::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_accept_offer_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "AuctionStarted" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = AuctionStarted::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_auction_started_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "Bid" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = Bid::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_auction_bid_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                "AuctionEnded" => {
                    let data = dynamic_event.to_ces_bytes().unwrap();
                    let (parsed_further, rem) = AuctionEnded::from_bytes(&data).unwrap(); // TODO
                    assert!(rem.len() == 0);

                    println!("-> {:?}", parsed_further);
                    let result = write_auction_ended_event_to_db(client, &parsed_further, event_id).await;
                    
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            
                        }
                    }
                }
                _ => {
                    println!("Unknown event type")
                }
            }

            tokio::time::sleep(Duration::from_millis(2000)).await;

            event_id += 1;
        } else {
            tokio::time::sleep(Duration::from_millis(10000)).await;
        }
    }
}

