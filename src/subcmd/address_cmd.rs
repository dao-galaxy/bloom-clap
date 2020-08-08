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
use tiny_keccak::{Hasher, Keccak};

pub fn address_processor(opt_match : Option<&ArgMatches>) {
    println!("##address##");
    if let Some(matches) = opt_match {
        let address_type = matches.value_of("type").unwrap();
        match address_type {
            "eth" => {
                if matches.occurrences_of("public") == 1 {
                    let pub_key : H512 = matches.value_of("public").unwrap().parse().unwrap();
                    let address = public_to_address(&pub_key);
                    println!("[public key] => External address: {}", hex::encode(address));
                } else if matches.occurrences_of("sender") == 1 {
                    let sender : H160 = matches.value_of("sender").unwrap().parse().unwrap();
                    let c_addr: CA;
                    if matches.occurrences_of("nonce") == 1 {
                        let nonce : u128 = matches.value_of("nonce").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_and_nonce(&sender, &U256::from(nonce));
                        println!("[sender, nonce] => Contract address: {}", hex::encode(c_addr.0));
                    } else if matches.occurrences_of("salt") == 1 {
                        let salt : H256 = matches.value_of("salt").unwrap().parse().unwrap();
                        let code_hash : H256 = matches.value_of("code-hash").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_salt_and_code(&sender, salt, code_hash);
                        println!("[sender, salt, code-hash] => Contract address: {}", hex::encode(c_addr.0));
                    } else {
                        let code_hash : H256 = matches.value_of("code-hash").unwrap().parse().unwrap();
                        c_addr = CA::from_sender_and_code(&sender, code_hash);
                        println!("[sender, code-hash] => Contract address: {}", hex::encode(c_addr.0));
                    }
                } else if matches.occurrences_of("checksum") == 1 {
                    let checksum : H160 = matches.value_of("checksum").unwrap().parse().unwrap();
                    let str_addr = hex::encode(checksum);
                    if !eip_55_adress_validate(&str_addr) {
                        println!("Checksum invalid, it should be {}", eip_55_address_checksum(&str_addr));
                    } else {
                        println!("Checksum OK");
                    }
                }
            },
            "btc" | _ => {
                println!("Address type \"{}\" not supported yet.", address_type);
            }
        };
    }

}

pub fn eip_55_adress_validate(address: &str) -> bool {
    let check = eip_55_address_checksum(address);
    check == address
}

pub fn eip_55_address_checksum(address: &str) -> String {
    let input = String::from(address.to_ascii_lowercase().trim_start_matches("0x"));
    let hash = hex::encode(keccak256_array(input.as_bytes()));
    let mut ret = String::with_capacity(42);
    ret.push_str("0x");
    for i in 0..40 {
        if u32::from_str_radix(&hash[i..i+1], 16).unwrap() >= 8 {
            ret.push_str(&input[i..i+1].to_ascii_uppercase());
        } else {
            ret.push_str(&input[i..i+1]);
        }
    }
    ret
}

pub fn keccak256_array(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

#[test]
fn test() {
    println!("{:?}", u32::from_str_radix("A", 16).unwrap());
    println!("{:?}", u32::from_str_radix("A", 16));
    assert_eq!(u32::from_str_radix("A", 16), Ok(10));

    println!("{}", eip_55_address_checksum("0x3a7b653e26f54e4a579237a15893e13a4bdd3451"));
    assert!(eip_55_adress_validate("0x3a7b653E26f54E4A579237A15893E13A4bDD3451"));

    println!("{}", eip_55_address_checksum("0xe0fc04fa2d34a66b779fd5cee748268032a146c0"));
    assert!(eip_55_adress_validate("0xe0FC04FA2d34a66B779fd5CEe748268032a146c0"));
}