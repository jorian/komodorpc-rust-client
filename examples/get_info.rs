extern crate komodo_rpc_client;

use komodo_rpc_client::KomodoClient;
use komodo_rpc_client::KomodoRpcApi;

pub fn main() {
    let rpc_user = "<rpcuser>";
    let rpc_password = "<rpcpassword>";
    let rpc_url = "http://127.0.0.1:7771";

    let komodo_client = KomodoClient::new(rpc_url, rpc_user, rpc_password);

    let info = komodo_client.get_info();

    match info {
        Err(rpc_error) => println!("{:?}", rpc_error),
        Ok(client_response) => {
            match client_response {
                Err(error) => println!("{}", error.message),
                Ok(info) => println!("{:?}", info)
            }
        }
    }
}