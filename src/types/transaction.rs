use BlockHash;
use TransactionId;

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
    pub vjoinsplit: Vec<String>,
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
    /// send|receive|immature|generate|orphan TODO: Create enum if needed
    pub category: String,
    pub amount: f64,
    pub fee: Option<f64>,
    pub vout: u32,
    #[serde(rename = "involvesWatchonly")]
    pub involves_watchonly: Option<bool>,
    pub abandoned: Option<bool>,
    pub size: u32,
}
