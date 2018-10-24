mod info;
#[macro_use]
mod from_str;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BlockHash(String);

from_str!(BlockHash);

pub use self::{info::*};