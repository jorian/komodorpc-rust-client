extern crate komodo_rpc_client;

use komodo_rpc_client::Client;
use komodo_rpc_client::KomodoRpcApi;

pub fn main() {
    let komodo_client = Client::new_komodo_client().unwrap();

    let info = komodo_client.get_info();

    match info {
        Err(rpc_error) => println!("{:?}", rpc_error),
        Ok(client_response) => {
            match client_response {
                Err(error) => println!("{}", error.message),
                Ok(info) => println!("{:?}", info)
      // or use Ok(info) => println!("{:?}", info.balance) to get balance, etc.
            }
        }
    }
}