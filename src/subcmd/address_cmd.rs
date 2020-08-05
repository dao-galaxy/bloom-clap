use clap::ArgMatches;
use hex::encode;
use primitive_types::{H160, H256, H512};
use contract_address::{
    Address,
    U256,
    ContractAddress as CA
};
use std::str::FromStr;
use parity_crypto::publickey::{Signature, Secret, Public, recover, public_to_address};
pub fn address_processor(opt_match : Option<&ArgMatches>) {
    println!("##address##");
    if let Some(matches) = opt_match {
        let address_type = matches.value_of("type").unwrap();
        match address_type {
            "eth" => {
                if matches.occurrences_of("public") > 0 {
                    let pub_key : H512 = matches.value_of("public").unwrap().parse().unwrap();
                    let address = public_to_address(&pub_key);
                    println!("[public key] => address: {}", hex::encode(address));
                } else {
                    let sender : H160 = matches.value_of("sender").unwrap().parse().unwrap();
                    let c_addr: CA;
                    if matches.occurrences_of("nonce") > 0 {
                        let nonce : u128 = matches.value_of("nonce").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_and_nonce(&sender, &U256::from(nonce));
                        println!("[sender, nonce] => address: {}", hex::encode(c_addr.0));
                    } else if matches.occurrences_of("salt") > 0{
                        let salt : H256 = matches.value_of("salt").unwrap().parse().unwrap();
                        let code_hash : H256 = matches.value_of("code-hash").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_salt_and_code(&sender, salt, code_hash);
                        println!("[sender, salt, code-hash] => address: {}", hex::encode(c_addr.0));
                    } else {
                        let code_hash : H256 = matches.value_of("code-hash").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_and_code(&sender, code_hash);
                        println!("[sender, code-hash] => address: {}", hex::encode(c_addr.0));
                    }
                }
            },
            "btc" | _ => {
                println!("Address type \"{}\" not supported yet.", address_type);
            }
        };
    }

}

