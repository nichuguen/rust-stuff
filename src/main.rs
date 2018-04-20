extern crate web3;
extern crate serde_json;

use serde_json::{Value};

use std::slice::Iter;

use web3::DuplexTransport;
use web3::futures::{Future, Stream};
use web3::types::Address;
use web3::contract::Contract;
use web3::types::{Bytes, H256, Log, H160, FilterBuilder,BlockNumber};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    match main_all()
    {
    Ok(_) => println!("Okay"),
    e => println!("Something failed: {:?}", e),
    }
}

/// runs all the examples
fn main_all() -> Result<(), String>{
    
    //let (_eloop, ws) = web3::transports::WebSocket::new("ws://localhost:8545").unwrap();
    let (_eloop, ws) = web3::transports::WebSocket::new("ws://159.100.254.244:8546").unwrap();

    let web3 = web3::Web3::new(ws.clone());

    // Prints some info about the account
//    main_test(&web3)?;

    // Prints new block headers
//    main_headers(&web3)?;

    // searching for this event
    //let event_signature = "TruiteFired(uint32)";
    //Transfer(address indexed _from, address indexed _to, uint256 _value)
    let event_signature = "Transfer(address,address,uint256)";
    let hashed_topic = web3.web3().sha3(Bytes(event_signature.as_bytes().to_vec())).wait().unwrap();
                                       
    // Filter with this address
    //let deployed_address = H160::from("0x5bab72b63bf38fa9e155dba7e5bdeebddb94c1e1"); // local truite 
    // test Golem Network Token
    let deployed_address = H160::from("0x7295bb8709ec1c22b758a8119a4214ffed016323"); 

    // prints logs filtered using hashed_topic and deployed_address
    main_logs(&web3, hashed_topic, deployed_address)?;

    // prints transactions as they arrive to the node
//    main_transactions(&web3)?;

    drop(web3);   
    Ok(())
}

/// Prints the 5 next BlockHeaders that are seen by the node
fn main_headers<T: DuplexTransport>(web3: &web3::Web3<T>) -> Result<(), String>{
    // the event contains a BlockHeader object
    let mut sub_headers = web3.eth_subscribe().subscribe_new_heads().wait().unwrap();
   
     (&mut sub_headers)
        .take(5)
        .for_each(|x| {
            println!("Got a new block header: {:?}", x);
            Ok(())
        })
        .wait()
        .unwrap();

    sub_headers.unsubscribe();

    Ok(())
}

/// Prints the next 5 events received by the node
/// Prints only the events that correpond to the hashed_topic parameter
/// Prints only the events that are on the deployed_address contract
///
fn main_logs<T: DuplexTransport>(web3: &web3::Web3<T>, hashed_topic: H256, deployed_address: H160) -> Result<(), String>{

    println!("Looking for a topic with this hash: {:?}", hashed_topic);
    println!("Looking for events from this contract: {:?}", deployed_address); 
    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Latest)
        .topics(Some(vec![hashed_topic]), None, None, None)
// see https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_newfilter and http://web3py.readthedocs.io/en/stable/filters.html#event-log-filters
// this should work but does not return anything:
//      .address(vec![deployed_address])
        .build();

    println!("Filter: {:?}", filter);

    let mut sub_logs = web3.eth_subscribe().subscribe_logs(filter).wait().unwrap();

    (&mut sub_logs)
        .take(5)
        .for_each(|log|{
            println!("Log: {:?}", log);
            if log.address == deployed_address{
                println!("should get in there");
            }
            else{
                println!("should NOT get in there");
            }
            Ok(())
        })
    .wait()
    .unwrap();;

    sub_logs.unsubscribe();
    Ok(())
}

/// Prints the next 5 pending transactions
fn main_transactions<T: DuplexTransport>(web3: &web3::Web3<T>) -> Result<(), String>{
    // the event contains the transaction ID
    let mut sub_transactions = web3.eth_subscribe().subscribe_new_pending_transactions().wait().unwrap();

    (&mut sub_transactions)
        .take(5) // remove this for endless loop
        .for_each(|x| {
            println!("x: {:?}",x );
            Ok(())
        })
        .wait()
        .unwrap();

    sub_transactions.unsubscribe();

    Ok(())
}

/// prints the abi of an example contract, and the account list of the node
fn main_test<T: DuplexTransport>(web3: &web3::Web3<T>) -> Result<(), String>{
    let abi = match extract_abi("./truffle/build/contracts/Truite.json")
    {
        Ok(s) => s,
        e => return Err(format!("Error reading abi {:?}", e)),
    };

    println!("Abi: {:?}", abi);

    let accounts = match web3.eth().accounts().wait() {
        Ok(a) => a,
        e => return Err(format!("Error getting account list {:?}", e)),
    };
    
    println!("Accounts: {:?}", accounts);

    Ok(())
}

/// extracts the json formatted string coresponding to the abi
/// the input_filename is a path to a truffle-compiled file
fn extract_abi(input_filename: &str) -> Result<String, serde_json::Error>
{
    let mut f = File::open(input_filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    let json: Value = serde_json::from_str(&contents).unwrap();

    let j = serde_json::to_string(&json["abi"])?;
    Ok(j)
}


//fn extract_bytecode(input_filename: &str) -> Result<Vec<u8>, serde_json::Error>
//{
//    println!("input file {}", input_filename);
//    let mut f = File::open(input_filename).expect("File not found");
//
//    let mut contents = String::new();
//    f.read_to_string(&mut contents)
//        .expect("Something went wrong reading the file.");
//
//    let json: Value = serde_json::from_str(&contents).unwrap();
//
//    let j = serde_json::to_string(&json["bytecode"])?;
//    Ok(j.bytes())
//}
