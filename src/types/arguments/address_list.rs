// input to several AddressIndex RPCs
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressList {
    pub addresses: Vec<String>,
}

impl AddressList {
    pub fn new() -> Self {
        AddressList { addresses: vec![] }
    }

    pub fn add(&mut self, address: &str) {
        self.addresses.push(address.parse().unwrap())
    }
}
