use serde_json;
use std::collections::HashMap;

//#[derive(Debug, Deserialize, Serialize)]
//pub struct CreateRawTransactionInputs {
//    txid: String,
//    vout: u32,
//}
//
//impl CreateRawTransactionInputs {
//    pub fn new() -> Self {
//        CreateRawTransactionInputs { }
//    }
//}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRawTransactionOutputs(HashMap<String, f64>);

impl CreateRawTransactionOutputs {
    pub fn new() -> Self {
        CreateRawTransactionOutputs(HashMap::new())
    }

    pub fn add(&mut self, address: &str, amount: f64) {
        self.0.insert(address.to_owned(), amount);
    }
}


