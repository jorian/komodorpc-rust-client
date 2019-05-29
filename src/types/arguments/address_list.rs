use arguments::address::Address;

// input to several AddressIndex RPCs
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressList {
    pub addresses: Vec<Address>,
}

impl AddressList {
    pub fn new() -> Self {
        AddressList { addresses: vec![] }
    }

    pub fn add(&mut self, address: &str) {
        self.addresses.push(Address::from(address).unwrap())
    }

    pub fn from(address: &str) -> Self {
        AddressList {
            addresses: vec![Address::from(address).unwrap()]
        }
    }

    pub fn from_address(address: &Address) -> Self {
        AddressList {
            addresses: vec![address.clone()]
        }
    }
}
