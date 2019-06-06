use BlockHash;
use TransactionId;
use ScriptPubKey;

use std::time::SystemTime;
use types::arguments::address::Address;

#[derive(Deserialize, Serialize, Debug)]
pub struct SerializedRawTransaction(pub String);

impl SerializedRawTransaction {

    /// This method sets the locktime of a transaction.
    /// A SerializedRawTransaction upon creation does not have a set locktime.
    /// To earn KMD rewards, it must be set to the current time - 777 seconds in little endian hex.
    /// The first 8 chars of the last 38 chars of the hex string is the place to set the locktime.
    /// NOTE: this only applies to KMD, not its assetchains.
    pub fn set_locktime(&mut self) {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // todo return result
        let hex_time = format!("{:x}", current_time.as_secs() - 120);

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

    pub fn from_hex(hex: String) -> Self {
        SerializedRawTransaction(hex.clone())
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
    #[serde(rename = "scriptSig")]
    pub script_sig: ScriptSig,
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
    pub interest: Option<f64>,
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
    #[serde(rename = "onetimePubKey")]
    pub onetime_pubkey: String,
    #[serde(rename = "randomSeed")]
    pub random_seed: String,
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

impl SignedRawTransaction {
    pub fn to_string(&self) -> String {
        self.hex.clone()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignRawTransactionError {
    pub txid: String,
    pub vout: u16,
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
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
    AllAnyoneCanPay,
    #[serde(rename = "NONE|ANYONECANPAY")]
    NoneAnyoneCanPay,
    #[serde(rename = "SINGLE|ANYONECANPAY")]
    SingleAnyoneCanPay,
}

#[derive(Deserialize, Debug)]
pub struct Unspent {
    txid: TransactionId,
    vout: u8,
    generated: bool,
    address: Address,
    account: Option<String>, //does not exist for an utxo in a change address
    amount: f64,
    interest: Option<f64>,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    rawconfirmations: u64,
    confirmations: u64,
    spendable: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LockedUnspent {
    pub txid: TransactionId,
    pub vout: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReceivedByAddress {
    address: Address,
    account: Option<String>, //accounts are deprecated
    amount: f64,
    rawconfirmations: u64,
    confirmations: u64,
    txids: Vec<TransactionId>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxListSinceBlock {
    transactions: Vec<ListTransaction>,
    lastblock: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListTransaction {
    account: String,
    address: Address,
    category: TxCategory,
    amount: f64,
    vout: u32,
    fee: Option<f64>,
    rawconfirmations: u64,
    confirmations: u64,
    blockhash: String,
    blockindex: u64,
    blocktime: u64,
    expiryheight: u64,
    txid: TransactionId,
    walletconflicts: Vec<Option<String>>,
    time: u64,
    timereceived: u64,
    vjoinsplit: Vec<Option<VJoinsplit>>,
    size: u32,
    comment: Option<String>,
    to: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TxCategory {
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "receive")]
    Receive,
    #[serde(rename = "move")]
    Move
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListTransactions(Vec<ListTransaction>);

#[derive(Deserialize, Serialize, Debug)]
pub struct ResentWalletTransactions(Vec<TransactionId>);

#[derive(Deserialize, Serialize, Debug)]
pub struct SetPubkey {
    pubkey: String,
    ismine: bool,
    address: Address,
}