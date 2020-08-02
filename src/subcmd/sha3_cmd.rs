use clap::ArgMatches;
use hex::encode;
use primitive_types::{H160, H256};
use sha3::{Digest, Sha3_256};

pub fn sha3_processor(opt_match : Option<&ArgMatches>) {
    println!("##sha3##");
    if let Some(matches) = opt_match {
        let tx = hex::decode(matches.value_of("hex").unwrap()).unwrap();
        let mut bee = sha3_256(tx.as_slice());
        let ret = H256::from(bee);
        println!("{:x}", ret);
    }
}

/// Do a keccak 256-bit hash and return result.
pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(data);
    // read hash digest
    let result = hasher.finalize();
    let mut bee = [0u8; 32];
    bee.copy_from_slice(result.as_slice());
    bee
}
