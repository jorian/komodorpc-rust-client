extern crate komodo_rpc_client;

use komodo_rpc_client::Client;
use komodo_rpc_client::KomodoRpcApi;
use komodo_rpc_client::arguments::AddressList;
use std::error::Error;

pub fn main() {
    let komodo_client = Client::new_komodo_client().expect("Failed to start KMD client");

    let mut list_of_addresses = AddressList::new();
    list_of_addresses.add("RBpNURYPcr2yDDAX3UEVKHNvkMfXMMmRDZ");

    let address_txids = komodo_client.get_address_tx_ids(&list_of_addresses);

    match address_txids {
        Err(e) => println!("{:?}", e.description()),
        Ok(res) => println!("{:?}", res),
    }
}