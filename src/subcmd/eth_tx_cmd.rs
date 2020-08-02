
use clap::ArgMatches;
use hex::encode;
use primitive_types::{H160, H256, U256};
use sha3::{Digest, Sha3_256};

use num_traits::int;
use rlp::RlpStream;
use secp256k1::{key::SecretKey, Message, Secp256k1};
use tiny_keccak::{Keccak, Hasher};

pub fn eth_tx_processor(opt_match : Option<&ArgMatches>) {
    println!("##eth-tx##");
    if let Some(matches) = opt_match {
        let to = hex::decode(matches.value_of("to").unwrap()).unwrap();
        let mut bee : [u8; 20] = Default::default();
        bee.copy_from_slice(to.as_slice());
        let to = H160::from(bee);

        let mut key: [u8; 32] = Default::default();
        key.copy_from_slice(&hex::decode(matches.value_of("private-key").unwrap()).unwrap());
        let private_key = H256::from(key);

        let data  = hex::decode(matches.value_of("data").unwrap_or("")).unwrap();
        let value : u128 = matches.value_of("value").unwrap_or("0").parse().unwrap();
        let gas_price : u128 = matches.value_of("gas-price").unwrap_or("1").parse().unwrap();

        // !!! Using U256 directerly is OK, but the argument will be treated as a hex-string (not a decimal-string) in parse().
        // let nonce : U256 = matches.value_of("nonce").unwrap().parse().unwrap(); // !!! nonce be parsed as a hex-string.
        let nonce : u128 = matches.value_of("nonce").unwrap().parse().unwrap(); // !!! nonce be parsed as a decimal-string.
        let gas : u128 = matches.value_of("gas").unwrap().parse().unwrap();
        let chain_id : u32 = matches.value_of("chain-id").unwrap_or("1").parse().unwrap();

        let raw_tx = RawTransaction {
            nonce : U256::from(nonce),
            to : Some(to),
            value : U256::from(value),
            gas_price : U256::from(gas_price),
            gas : U256::from(gas),
            data : data
        };
        let raw_rlp_bytes = raw_tx.sign(&private_key, &chain_id);
        println!("{}", hex::encode(raw_rlp_bytes));
    }
}


pub struct EcdsaSig {
    v: u64,
    r: Vec<u8>,
    s: Vec<u8>,
}

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction {
    /// Nonce
    pub nonce: U256,
    /// Recipient (None when contract creation)
    pub to: Option<H160>,
    /// Transfered value
    pub value: U256,
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub data: Vec<u8>,
}

impl RawTransaction {
    /// Signs and returns the RLP-encoded transaction
    pub fn sign<T: int::PrimInt>(&self, private_key: &H256, chain_id: &T) -> Vec<u8> {
        let chain_id_u64: u64 = chain_id.to_u64().unwrap();
        let hash = self.hash(chain_id_u64);
        let sig = ecdsa_sign(&hash, &private_key.0, &chain_id_u64);
        let mut r_n = sig.r;
        let mut s_n = sig.s;
        while r_n[0] == 0 {
            r_n.remove(0);
        }
        while s_n[0] == 0 {
            s_n.remove(0);
        }
        let mut tx = RlpStream::new();
        tx.begin_unbounded_list();
        self.encode(&mut tx);
        tx.append(&sig.v);
        tx.append(&r_n);
        tx.append(&s_n);
        tx.finalize_unbounded_list();
        tx.out()
    }

    fn hash(&self, chain_id: u64) -> Vec<u8> {
        let mut hash = RlpStream::new();
        hash.begin_unbounded_list();
        self.encode(&mut hash);
        hash.append(&chain_id.clone());
        hash.append(&U256::zero());
        hash.append(&U256::zero());
        hash.finalize_unbounded_list();
        keccak256_hash(&hash.out())
    }

    fn encode(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let Some(ref t) = self.to {
            s.append(t);
        } else {
            s.append(&vec![]);
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

fn keccak256_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut resp: [u8; 32] = Default::default();
    hasher.finalize(&mut resp);
    resp.iter().cloned().collect()
}

fn ecdsa_sign(hash: &[u8], private_key: &[u8], chain_id: &u64) -> EcdsaSig {
    let s = Secp256k1::signing_only();
    let msg = Message::from_slice(hash).unwrap();
    let key = SecretKey::from_slice(private_key).unwrap();
    let (v, sig_bytes) = s.sign_recoverable(&msg, &key).serialize_compact();

    EcdsaSig {
        v: v.to_i32() as u64 + chain_id * 2 + 35,
        r: sig_bytes[0..32].to_vec(),
        s: sig_bytes[32..64].to_vec(),
    }
}


mod test {

    #[test]
    fn test_signs_transaction_eth() {
        use primitive_types::{H160, H256, U256};
        use super::RawTransaction;
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
        use super::RawTransaction;
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

    #[test]
    fn test_signs_transaction_ropsten2() {
        use primitive_types::{H160, H256, U256};
        use super::RawTransaction;
        // 1 mainnet, 3 ropsten
        const ETH_CHAIN_ID: u32 = 3;

        let tx = RawTransaction {
            nonce: U256::from(0),
            to: Some(H160::zero()),
            value: U256::zero(),
            gas_price: U256::from(10000),
            gas: U256::from(21240),
            data: hex::decode(
                "7f7465737432000000000000000000000000000000000000000000000000000000600057"
            ).unwrap(),
        };

        let mut data: [u8; 32] = Default::default();
        data.copy_from_slice(&hex::decode("2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4").unwrap());
        let private_key = H256(data);
        let raw_rlp_bytes = tx.sign(&private_key, &ETH_CHAIN_ID);

        let result = "f885808227108252f894000000000000000000000000000000000000000080a\
                            47f746573743200000000000000000000000000000000000000000000000000\
                            00006000572aa0b4e0309bc4953b1ca0c7eb7c0d15cc812eb4417cbd759aa09\
                            3d38cb72851a14ca036e4ee3f3dbb25d6f7b8bd4dac0b4b5c717708d20ae6ff\
                            08b6f71cbf0b9ad2f4";
        println!("{}", result);
        assert_eq!(result, hex::encode(raw_rlp_bytes));

    }

}



