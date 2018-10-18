extern crate komodotest;

use komodotest::KomodoClient;

pub fn main() {
    println!("Hello!");

    let komodo_client = KomodoClient::new("http://127.0.0.1:7771", "komodouser", "4j2kgkljtFYHGL56R57Df568d5687D57D97DG67h8");

    let result: Result<Result<Info, _>, _> = komodo_client.get_info();

    println!("{:?}", result.unwrap())
}

