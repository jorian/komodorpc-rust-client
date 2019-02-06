use std::collections::HashMap;
use TransactionId;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRawTransactionInputs(Vec<Input>);

impl CreateRawTransactionInputs {
    pub fn new() -> Self {
        CreateRawTransactionInputs(Vec::new())
    }

    pub fn add(&mut self, txid: &str, vout: u32) {
        let input = Input {
            txid: txid.to_owned(),
            vout
        };

        self.0.push(input);
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    txid: String,
    vout: u32
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionOutputDetail {
    pub txid: TransactionId,
    pub vout: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>, // is hex hash
    pub amount: f64,
}