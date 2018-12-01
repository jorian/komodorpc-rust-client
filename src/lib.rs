// bring all libraries used into scope:
extern crate base64;
extern crate bitcoin;
extern crate hex as std_hex;
//extern crate jsonrpc_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate core;
extern crate dirs;
extern crate reqwest;
extern crate os_info;

mod rpc;
mod komodocore;
mod komodo_rpc_api;
mod chains;
mod error;
mod rpcconn;

pub type TransactionId = bitcoin::util::hash::Sha256dHash;
pub type BlockHash = bitcoin::util::hash::Sha256dHash;

pub use komodo_rpc_api::KomodoRpcApi;
pub use komodocore::Client;

pub use error::ApiError;

pub use rpc::*; // this basically eliminates mentioning 'rpc'
pub use rpcconn::*;

pub use bitcoin::network::constants::Network;
pub use bitcoin::util::privkey::Privkey;
pub use bitcoin::Script;

pub use chains::Chain;