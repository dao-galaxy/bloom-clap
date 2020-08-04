use clap::ArgMatches;
use hex::encode;
use primitive_types::{H160, H256};
use sha3::{Digest, Sha3_256};

use contract_address::{
    Address, U256, ContractAddress
};
use std::str::FromStr;



pub fn address_processor(opt_match : Option<&ArgMatches>) {
    println!("##address##");
    let sender = Address::from_str("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6").unwrap();
    let contract_address = ContractAddress::from_sender_and_nonce(&sender, &U256::zero());
    println!("{:?}", contract_address);
}



