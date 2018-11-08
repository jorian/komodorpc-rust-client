

// getchaintips has weird behaviour: first 2 lines of string, then the actual JSON array.
// todo: find out how to ignore stuff in serde.

#[derive(Debug, Deserialize)]
pub struct ChainTips {
    pub iterations: String,
    pub chaintips: Vec<ChainTip>
}

#[derive(Debug, Deserialize)]
pub struct ChainTip {
    pub height: u64,
    pub hash: String,
    pub branchlen: u32,
    #[serde(flatten)]
    pub status: ChainTipStatus
}

#[derive(Debug, Deserialize)]
pub enum ChainTipStatus {
    Invalid     (String),
    HeadersOnly (String),
    ValidHeaders(String),
    ValidFork   (String),
    Active      (String)
}