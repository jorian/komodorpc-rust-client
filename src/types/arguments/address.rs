//pub enum Payload {
//    Pubkey(String),
//    PubkeyHash(String),
//    ScriptHash(String)
//}
//
// TODO: need to properly deserialize info from daemon
//// TODO: need to read up on deserializing with Serde
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct Address {
//    // the bitcoin library uses a payload type, which determines p2ph, p2pkh, scripthash or segwit address.
//    // Komodo doesn't use segwit, so we can skip that one.
//
//    // TODO: for now use String
//    // pub payload: Payload
//    pub payload: String
//}
//
//impl Address {
//    // this method should take a reference to a pubkey to calculate the correct base58 KMD address
//    pub fn p2pkh() -> Address {
//        unimplemented!();
//    }
//
//    // TODO: support for uncompressed?
//
//    // TODO: support for p2pk?
//    // seems to be early day addresses
//
//    // this method takes a script, and calculates an address based on this script:
//    pub fn p2sh() -> Address {
//        unimplemented!();
//    }
//}

use ApiError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use std::convert::TryFrom;
use std::cmp;

/// Address is either Transparent (address starts with `R`) or Shielded (all sapling, starts with `zs`)
#[derive(Debug, Clone, Hash, Eq)]
pub struct Address {
    pub(crate) addr: String,
    pub(crate) addr_type: AddrType,
}

impl TryFrom<&String> for Address {
    type Error = ApiError;

    fn try_from(addr_str: &String) -> Result<Self, Self::Error> {
        match addr_str.len() {
            78 => Ok(Address {
                addr: addr_str.to_string(),
                addr_type: AddrType::Shielded,
            }),
            34 => Ok(Address {
                addr: addr_str.to_string(),
                addr_type: AddrType::Transparent,
            }),
            _ => Err(ApiError::Other(format!("Address has incorrect length")))
        }
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr.eq(&other.addr)
    }
}

//impl Eq for Address { }

impl Address {
    // todo check if address is correctly encoded
    pub fn from(addr_str: &str) -> Result<Address, ApiError> {
        match addr_str.len() {
            78 => Ok(Address {
                    addr: addr_str.to_string(),
                    addr_type: AddrType::Shielded,
                }),
            34 => Ok(Address {
                    addr: addr_str.to_string(),
                    addr_type: AddrType::Transparent,
                }),
            _ => Err(ApiError::Other(format!("Address has incorrect length")))
        }
    }

    /// for use in `z_shieldcoinbase` to merge all coinbases to a Shielded address
    pub fn any() -> Address {
        Address {
            addr: "*".to_string(),
            addr_type: AddrType::Transparent
        }
    }

    pub fn to_string(&self) -> String {
        self.addr.clone()
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Address::from(s)
            .map_err(D::Error::custom)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.serialize_str(&self.addr)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum AddrType {
    Transparent,
    Shielded
}

// for use in `z_mergetoaddress` RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct FromAddresses(pub(crate) Vec<String>);

impl FromAddresses {
    // if not used, assume wildcard
    pub fn from(addresses: Vec<Address>) -> FromAddresses {
        let mut result: Vec<String> = vec![];
        for addr in addresses {
            result.push(
                addr.addr.clone()
            );
        }

        FromAddresses { 0: result }
    }

    pub fn all() -> FromAddresses {
        FromAddresses { 0: vec!["*".to_string() ]}
    }

    pub fn any_taddr() -> FromAddresses {
        FromAddresses { 0: vec!["ANY_TADDR".to_string()] }
    }

    pub fn any_zaddr() -> FromAddresses {
        FromAddresses { 0: vec!["ANY_ZADDR".to_string()] }
    }
}

// for use in `z_shieldcoinbase` RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct FromAddress(String);

impl FromAddress {
    // if not used, assume wildcard
    pub fn from(address: Address) -> FromAddress {
        FromAddress { 0: address.addr }
    }
}

// for use in `z_sendmany` RPC
pub struct Amounts(pub(crate) Vec<Amount>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Amount {
    pub address: Address,
    pub amount: f64,
    pub memo: Option<String>,
}

impl From<Vec<Amount>> for Amounts {
    fn from(v: Vec<Amount>) -> Self {
        let mut result = vec![];
        for amount in v {
            result.push(amount.clone())
        };

        Amounts { 0: result }
    }
}