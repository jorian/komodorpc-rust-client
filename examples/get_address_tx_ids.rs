extern crate komodo_rpc_client;

use komodo_rpc_client::KomodoClient;
use komodo_rpc_client::KomodoRpcApi;
use komodo_rpc_client::arguments::AddressList;

pub fn main() {
    let rpc_user = "<rpcuser>";
    let rpc_password = "<rpcpassword>";
    let rpc_url = "http://127.0.0.1:7771";

    let komodo_client = KomodoClient::new(rpc_url, rpc_user, rpc_password);

    let mut list_of_addresses = AddressList::new();
    list_of_addresses.add("RBpNURYPcr2yDDAX3UEVKHNvkMfXMMmRDZ");

    let address_txids = komodo_client.get_address_tx_ids(&list_of_addresses);

    match address_txids {
        Err(e) => println!("{:?}", e),
        Ok(ok) => {
            match ok {
                Err(rpcerror) => println!("{:?}", rpcerror.message),
                Ok(txids) => println!("{:?}", txids),
            }
        }
    }
}