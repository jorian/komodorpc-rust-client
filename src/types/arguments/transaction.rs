use std::collections::HashMap;
use TransactionId;
use std::io::SeekFrom::Start;
use types::address_index::AddressUtxos;
use std::ops::Add;
use bitcoin::util::hash::Sha256dHash;
use ApiError;
use std::iter::FromIterator;

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

impl From<AddressUtxos> for CreateRawTransactionInputs {
    fn from(utxo_set: AddressUtxos) -> Self {
        let mut set = vec![];
        for utxo in &utxo_set.0 {
            set.push(Input {
                txid: utxo.txid.clone(),
                vout: utxo.output_index
            })
        }

        CreateRawTransactionInputs { 0: set }
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct P2SHInput {
    pub txid: TransactionId,
    pub vout: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>, // is hex hash
    pub amount: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct P2SHInputSet(pub Vec<P2SHInput>);

impl P2SHInputSet {
    pub fn builder<'a>() -> P2SHInputSetBuilder {
        P2SHInputSetBuilder {
            redeem_script: None,
            p2sh_input_set: None,
        }
    }
}

impl FromIterator<P2SHInput> for P2SHInputSet {
    fn from_iter<T: IntoIterator<Item=P2SHInput>>(iter: T) -> Self {
        let mut result = Vec::new();
        for i in iter {
            result.push(i);
        }

        P2SHInputSet { 0: result}
    }
}

pub struct P2SHInputSetBuilder {
    redeem_script: Option<String>,
    p2sh_input_set: Option<P2SHInputSet>,
}

impl P2SHInputSetBuilder {
    pub fn set_redeem_script(&mut self, redeem_script: String) -> &mut Self {
        self.redeem_script = Some(redeem_script.clone());

        self
    }

    pub fn build(&self) -> Result<P2SHInputSet, ApiError> {
        match self.redeem_script.clone() {
            Some(script) => {
                let mut v = self.p2sh_input_set.clone().unwrap();
                for i in &mut v.0 {
                    i.redeem_script = Some(script.clone())
                }

                Ok(v)
            },
            None => Err(ApiError::Other(String::from("Failed to build P2SH Inputs, redeem_script not set")))
        }
    }
}

impl From<&AddressUtxos> for P2SHInputSetBuilder {
    fn from(utxo_set: &AddressUtxos) -> Self {
        let mut set = vec![];
        for utxo in &utxo_set.0 {
            set.push(P2SHInput {
                // todo the unwrap here is ugly, but needed:
                txid: TransactionId::from(Sha256dHash::from_hex(&utxo.txid).unwrap()),
                vout: utxo.output_index,
                script_pub_key: utxo.script.clone(),
                amount: (utxo.satoshis as f64 / 100_000_000.0),
                redeem_script: None,
            })
        }

        P2SHInputSetBuilder {
            redeem_script: None,
            p2sh_input_set: Some(P2SHInputSet {
                0: set
            })
        }
    }
}