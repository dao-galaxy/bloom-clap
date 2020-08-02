



mod keccak_cmd;
mod sha3_cmd;
mod eth_tx_cmd;

pub use keccak_cmd::{keccak_processor};
pub use sha3_cmd::{sha3_processor};
pub use eth_tx_cmd::{eth_tx_processor};
pub use eth_tx_cmd::RawTransaction;

pub use clap::ArgMatches;

pub fn hash_processor(opt_match : Option<&ArgMatches>) {
    println!("##hash##\nkeccak256  sha3  blake2  sha256");
}


mod test {

    #[test]
    fn test_signs_transaction_eth() {
        use primitive_types::{H160, H256, U256};
        use super::eth_tx_cmd::RawTransaction;
        use serde_json;
        use std::fs::File;
        use std::io::Read;

        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256,
        }

        let mut file = File::open("./test/test_txs.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 1 as u64;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }

    #[test]
    fn test_signs_transaction_ropsten() {
        use primitive_types::{H160, H256, U256};
        use super::eth_tx_cmd::RawTransaction;
        use serde_json;
        use std::fs::File;
        use std::io::Read;
        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256,
        }

        let mut file = File::open("./test/test_txs_ropsten.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 3 as i32;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }
}
