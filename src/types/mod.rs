mod info;
#[macro_use]
mod from_str;

// https://stackoverflow.com/questions/52256104/how-to-instantiate-a-public-tuple-structwith-private-field-from-a-different-mo
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BlockHash(String);

from_str!(BlockHash);

impl BlockHash {
    pub fn to_str(&self) -> &str {
        self.0.as_str()
    } // little tryout
}

pub use self::{info::*};