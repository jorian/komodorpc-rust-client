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

/// Address is either Transparent (address starts with `R`) or Shielded (all sapling, starts with `zs`)
#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    addr: String,
    addr_type: AddrType,
}

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
}

#[derive(Deserialize, Serialize, Debug)]
enum AddrType {
    Transparent,
    Shielded
}

// for use in `z_mergetoaddress` RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct FromAddresses(Vec<String>);

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
}

// for use in `z_shieldcoinbase` RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct FromAddress(String);

impl FromAddress {
    // if not used, assume wildcard
    pub fn from(address: Address) -> FromAddress {
        FromAddress { 0: address.addr }
    }

    pub fn all() -> FromAddress {
        FromAddress { 0: format!("*") }
    }
}

// for use in `z_sendmany` RPC
pub struct Amounts(Vec<Amount>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Amount {
    pub addr: Address,
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