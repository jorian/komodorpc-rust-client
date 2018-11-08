mod transaction;
mod info;
mod blockchain;
mod address;
mod address_index;
#[macro_use]
mod from_str;

pub mod arguments;


// https://stackoverflow.com/questions/52256104/how-to-instantiate-a-public-tuple-structwith-private-field-from-a-different-mo
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BlockHash(String);

from_str!(BlockHash);

impl BlockHash {
    pub fn to_str(&self) -> &str {
        self.0.as_str()
    } // little tryout
}

pub use self::{info::*, transaction::*, blockchain::*, address::*, address_index::*};
