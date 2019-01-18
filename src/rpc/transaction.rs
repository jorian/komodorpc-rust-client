use BlockHash;
use TransactionId;
use ScriptPubKey;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Debug)]
pub struct SerializedRawTransaction(pub String);

impl SerializedRawTransaction {

    // This method sets the locktime of a transaction.
    // A SerializedRawTransaction upon creation does not have a set locktime.
    // To earn KMD rewards, it must be set to the current time - 777 seconds in little endian hex.
    // The first 8 chars of the last 38 chars of the hex string is the place to set the locktime.
    // NOTE: this only applies to KMD, not its assetchains.
    pub fn set_locktime(&mut self) {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // todo return result
        let hex_time = format!("{:x}", current_time.as_secs() - 777);

        let mut rev_hexified_time = String::new();

        // hex_time must be little endian
        for i in (0..8).rev().step_by(2) {
            rev_hexified_time.push_str(hex_time.get(i-1..i+1).unwrap());
        }

        let hex = self.0.clone();

        let payload = &hex[0..hex.len() - 38];
        let expiry = &hex[hex.len() - 30..hex.len() - 24];

        let mut result = String::new();
        result.push_str(payload);
        result.push_str(rev_hexified_time.as_str());
        result.push_str(expiry);
        result.push_str("000000000000000000000000");

        self.0 = result;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub amount: f64,
    pub confirmations: u32,
    pub blockhash: Option<BlockHash>,
    /// Unix Timestamp
    pub blockindex: Option<u64>,
    /// Unix Timestamp
    pub blocktime: Option<u64>,
    pub walletconflicts: Vec<TransactionId>,
    pub expiryheight: u32,
    pub txid: TransactionId,
    /// Unix Timestamp
    pub time: u64,
    /// Unix Timestamp
    pub timereceived: u64,
    pub vjoinsplit: Vec<VJoinsplit>,
    //    pub comment: Option<String>,
//    pub to: Option<String>,
//    #[serde(rename = "bip125-replaceable")]
    /// yes|no|unknown: TODO: Create enum if needed
//    pub bip125_replaceable: String,
    pub details: Vec<Detail>,
    pub hex: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Detail {
    pub account: String,
//    pub address: Option<Address>,
    pub address: String,
    // send|receive|immature|generate|orphan TODO: Create enum if needed
    pub category: String,
    pub amount: f64,
    pub fee: Option<f64>,
    pub vout: u32,
    #[serde(rename = "involvesWatchonly")]
    pub involves_watchonly: Option<bool>,
    pub abandoned: Option<bool>,
    pub size: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawTransaction {
    pub hex: Option<String>,
    pub overwintered: Option<bool>,
    pub txid: String,
    pub size: Option<u32>,
    pub versiongroupid: Option<String>,
    pub version: u32,
    pub locktime: u64,
    pub expiryheight: Option<u32>,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub vjoinsplit: Vec<VJoinsplit>,
    pub blockhash: Option<BlockHash>,
    pub confirmations: Option<u32>,
    pub time: Option<u64>,
    pub blocktime: Option<u64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vin {
    pub txid: TransactionId,
    pub vout: u32,
    pub scriptSig: ScriptSig,
    pub sequence: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vout {
    pub value: f64,
//    #[serde(rename = "valueSat")]
//    pub value_sat: Option<u64>,
//    // because of weirdness, valueZat is a possibility we need to account for too:
//    #[serde(rename = "valueZat")]
//    pub value_zat: Option<u64>,
    pub interest: f64,
    pub n: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptPubKey,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VJoinsplit {
    pub vpub_old: f64,
    pub vpub_new: f64,
    pub anchor: String,
    pub nullifiers: Vec<String>,
    pub commitments: Vec<String>,
    pub onetimePubKey: String,
    pub randomSeed: String,
    pub macs: Vec<String>,
    pub proof: String,
    pub ciphertexts: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignedRawTransaction {
    pub hex: String,
    pub complete: bool,
    pub errors: Option<Vec<SignRawTransactionError>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignRawTransactionError {
    pub txid: String,
    pub vout: u16,
    pub scriptSig: String,
    pub sequence: u64,
    pub error: String,
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

#[derive(Deserialize, Serialize, Debug)]
pub enum SigHashType {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ALL|ANYONECANPAY")]
    All_AnyoneCanPay,
    #[serde(rename = "NONE|ANYONECANPAY")]
    None_AnyoneCanPay,
    #[serde(rename = "SINGLE|ANYONECANPAY")]
    Single_AnyoneCanPay,
}