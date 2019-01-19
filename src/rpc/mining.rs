#[derive(Deserialize, Serialize, Debug)]
pub struct BlockSubsidy {
    pub miner: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockTemplate {
    pub capabilities: Vec<String>,
    pub version: u16,
    pub previousblockhash: String, // todo hash
    pub finalsaplingroothash: String, // todo hash
    pub transactions: Vec<IncludedTransaction>,
    pub coinbasetxn: CoinbaseTransaction,
    pub longpollid: String, // todo hash
    pub target: String,
    pub mintime: u64,
    pub mutable: Vec<String>,
    pub noncerange: String,
    pub sigoplimit: u64,
    pub sizelimit: u64,
    pub curtime: u64,
    pub bits: String,
    pub height: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CoinbaseTransaction {
    pub data: String,
    pub hash: String,
    pub depends: Vec<u64>,
    pub fee: i64,
    pub sigops: u64,
    pub coinbasevalue: u64,
    pub required: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IncludedTransaction {
    pub data: String,
    pub hash: String,
    pub depends: Vec<u64>,
    pub fee: u64,
    pub sigops: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestObject {
    pub mode: String,
    pub capabilities: Vec<String>
}