extern crate komodo_rpc_client;

use komodo_rpc_client::Client;
use komodo_rpc_client::KomodoRpcApi;

pub fn main() {
    let komodo_client = Client::new_komodo_client().unwrap();

    let info = komodo_client.get_info();

    match info {
        Err(api_error) => println!("{:?}", api_error),
        Ok(client_response) => {
            match client_response {
                Some(res) => println!("{:?}", res),
                None => println!("empty response"),
            }
        }
    }
}