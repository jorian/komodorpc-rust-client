extern crate komodotest;

use komodotest::KomodoClient;
use komodotest::BlockHash;

pub fn main() {
    println!("Hello!");

    let komodo_client = KomodoClient::new("http://127.0.0.1:7771", "komodouser", "4j2kgkljtFYHGL56R57Df568d5687D57D97DG67h8");

    let result: Result<Result<_, _>, _> = komodo_client.get_best_block_hash();

    println!("{:?}", result.unwrap())
}

