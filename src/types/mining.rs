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

#[derive(Deserialize, Serialize, Debug)]
pub struct MiningInfo {
    pub blocks: u32,
    pub currentblocksize: u32,
    pub currentblocktx: u32,
    pub difficulty: f64,
    pub errors:  String,
    pub genproclimit: i8,
    pub localsolps: f64,
    pub networksolps: u64,
    pub networkhashps: u64,
    pub pooledtx: u32,
    pub testnet: bool,
    pub chain: String,
    pub staking: bool,
    pub generate: bool,
    pub numthreads: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParametersObject {
    pub workid: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SubmitBlockResult {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "duplicate-invalid")]
    DuplicateInvalid,
    #[serde(rename = "duplicate-inconclusive")]
    DuplicateInconclusive,
    #[serde(rename = "inconclusive")]
    Inconclusive,
    #[serde(rename = "rejected")]
    Rejected
}