


use clap::ArgMatches;
use hex::encode;
use primitive_types::{H160, H256};
use tiny_keccak::{Keccak, Hasher};

pub fn keccak_processor(opt_match : Option<&ArgMatches>) {
    println!("##keccak256##");
    if let Some(matches) = opt_match {
        // println!("{}", matches.value_of("hex").unwrap());
        let tx = hex::decode(matches.value_of("hex").unwrap()).unwrap();
        let hash =  keccak256(&tx);
        let foo = H256::from(hash);
        println!("{:x}", foo);
    }
}

/// Do a keccak 256-bit hash and return result.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    keccak.update(data);
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    output
}


