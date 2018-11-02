// bring all libraries used into scope:
extern crate base64;
extern crate bitcoin;
extern crate jsonrpc_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use types::*; // this basically eliminates mentioning 'types'
pub use komodocore::*; // this basically eliminates mentioning 'komodocore'
pub use jsonrpc_client::{ClientError, RpcError};

mod types;
mod komodocore;
