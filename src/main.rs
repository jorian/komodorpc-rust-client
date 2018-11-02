extern crate komodorpc_rust_client;

use komodorpc_rust_client::KomodoClient;
use komodorpc_rust_client::BlockHash;
use komodorpc_rust_client::Info;

pub fn main() {
    let komodo_client = KomodoClient::new("http://127.0.0.1:7771", "komodouser", "4j2kgkljtFYHGL56R57Df568d5687D57D97DG67h8");

    let result: Result<Result<BlockHash, _>, _> = komodo_client.get_best_block_hash();
    let info: Result<Result<Info, _>, _> = komodo_client.get_info();

    println!("{}", result.unwrap().unwrap().to_str());

    match info.unwrap() {
        Err(e) => println!("{}", e.message),
        Ok(k) => println!("{:?}", k.blocks)
    }
}