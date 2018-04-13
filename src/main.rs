extern crate web3;
extern crate serde_json;

use serde_json::{Value};

use web3::futures::{Future, Stream};
use web3::types::Address;
use web3::contract::Contract;

use std::fs::File;
use std::io::prelude::*;

fn main_test() -> Result<(), String>{
    let abi = match extract_abi("./truffle/build/contracts/Truite.json")
    {
        Ok(s) => s,
        e => return Err(format!("Error reading abi {:?}", e)),
    };
    println!("ceci est le retour: {}", abi);


    let (_eloop, ws) = web3::transports::WebSocket::new("ws://localhost:8545").unwrap();
    let web3 = web3::Web3::new(ws.clone());
    let mut sub = web3.eth_subscribe().subscribe_new_heads().wait().unwrap();

    println!("Got subscription id: {:?}", sub.id());

    Contract::from_json(web3, Address::from("0xed6367b2b7e1f0a33cecd2d3b56fc6667a14e4a9"), abi.as_bytes());



    (&mut sub)
//        .take(5)
        .for_each(|x| {
            println!("Got: {:?}", x);
            Ok(())
        })
        .wait()
        .unwrap();

    sub.unsubscribe();

    drop(web3);   
    Ok(())
}

fn main() {
    match main_test()
    {
    Ok(_) => println!("Okay"),
    e => println!("Something failed: {:?}", e),
    }
}

fn extract_abi(input_filename: &str) -> Result<String, serde_json::Error>
{
    println!("input file {}", input_filename);
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
