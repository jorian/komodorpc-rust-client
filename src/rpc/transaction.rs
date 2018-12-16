use BlockHash;
use TransactionId;
use ScriptPubKey;

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
    #[serde(rename = "valueSat")]
    pub value_sat: u64,
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
