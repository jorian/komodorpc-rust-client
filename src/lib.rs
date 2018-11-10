// bring all libraries used into scope:
extern crate base64;
extern crate bitcoin;
extern crate hex as std_hex;
extern crate jsonrpc_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate core;
extern crate dirs;

mod rpc;
mod komodocore;
mod komodo_rpc_api;
mod assetchains;

pub type TransactionId = bitcoin::util::hash::Sha256dHash;

pub use komodo_rpc_api::KomodoRpcApi;
//pub use komodocore::*; // this basically eliminates mentioning 'komodocore'
pub use komodocore::Client as KomodoClient;
pub use komodocore::Client as ACclient;

pub use rpc::*; // this basically eliminates mentioning 'rpc'
pub use jsonrpc_client::{ClientError, RpcError};

pub use bitcoin::network::constants::Network;
pub use bitcoin::util::privkey::Privkey;
//pub use bitcoin::Address;
pub use bitcoin::Script;

pub use assetchains::Assetchain;