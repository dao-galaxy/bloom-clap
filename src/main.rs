#[macro_use]
extern crate hex_literal;

#[macro_use]
extern crate serde_derive;

mod subcmd;

use clap::{App, load_yaml, ArgMatches, Arg};
use subcmd::{keccak_processor, hash_processor, sha3_processor, eth_tx_processor};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");  // src/cli.yaml
    let matches = App::from(yaml).get_matches();
    // println!("{:#?}", matches);

    let (sub_cmd, opt_match)= matches.subcommand();
    match sub_cmd {
        "hash" => hash_processor(opt_match),
        "keccak256" => keccak_processor(opt_match),
        "sha3" => sha3_processor(opt_match),
        "eth-tx" => eth_tx_processor(opt_match),
        _ => {}
    }
    // more program logic goes here...
}

/*

https://cn.etherscan.com/getRawTx?tx=0x083cc4af906c0b8b67a630507f695aa0dab2bde84fada412fff608d0ee9ea1ae
f8ad830dd98a8502540be40083026739947c2af3a86b4bf47e6ee63ad9bde7b3b0ba7f95da80b844a9059cbb000000000000000000000000b34938746d316e995aa81f9b3f94419a0a41e14300000000000000000000000000000000000000000000026faff2dfe5c524000025a0167bf6ce1f7ecee1e5a414e3622baa14daf6caaf90f498b4fb94b1a91bc79491a0362191d3956065a0e14276dd4810b523e93a786091d27388a2b00b6955f93161

./target/debug/bloom-clap keccak256 f8ad830dd98a8502540be40083026739947c2af3a86b4bf47e6ee63ad9bde7b3b0ba7f95da80b844a9059cbb000000000000000000000000b34938746d316e995aa81f9b3f94419a0a41e14300000000000000000000000000000000000000000000026faff2dfe5c524000025a0167bf6ce1f7ecee1e5a414e3622baa14daf6caaf90f498b4fb94b1a91bc79491a0362191d3956065a0e14276dd4810b523e93a786091d27388a2b00b6955f93161
##keccak256##
083cc4af906c0b8b67a630507f695aa0dab2bde84fada412fff608d0ee9ea1ae

./target/debug/bloom-clap sha3 f8ad830dd98a8502540be40083026739947c2af3a86b4bf47e6ee63ad9bde7b3b0ba7f95da80b844a9059cbb000000000000000000000000b34938746d316e995aa81f9b3f94419a0a41e14300000000000000000000000000000000000000000000026faff2dfe5c524000025a0167bf6ce1f7ecee1e5a414e3622baa14daf6caaf90f498b4fb94b1a91bc79491a0362191d3956065a0e14276dd4810b523e93a786091d27388a2b00b6955f93161
##sha3##
485ea7d7f4a2ac351bbd088dd812a74a7f8f71719cb5a0650e18282ba3884347

https://www.ethereumdecoder.com/
./target/debug/bloom-clap eth-tx --nonce 0 --to 26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5 --value 0 --gas-price 10000 --gas 21240 --data 7f7465737432000000000000000000000000000000000000000000000000000000600057 --private-key 2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4 --chain-id 1

*/

