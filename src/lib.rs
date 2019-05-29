//extern crate base64;
extern crate bitcoin;
//extern crate hex as std_hex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate core;
extern crate dirs;
extern crate reqwest;
extern crate os_info;

mod types;
mod komodocore;
mod komodo_rpc_api;
mod chains;
mod error;
mod rpcconn;

pub type TransactionId = bitcoin::util::hash::Sha256dHash;

//impl TransactionId {
//    pub fn from_str(s: &str) -> Result<TransactionId, ApiError> {
//        let hash = Sha256dHash::from_hex(s)?;
//    }
//}

pub type BlockHash = bitcoin::util::hash::Sha256dHash;

pub use komodo_rpc_api::KomodoRpcApi;
pub use komodocore::Client;

pub use error::ApiError;

pub use types::*;
pub use rpcconn::*;

pub use bitcoin::network::constants::Network;
pub use bitcoin::util::privkey::Privkey;
pub use bitcoin::Script;
pub use chains::Chain;
