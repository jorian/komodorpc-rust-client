

// getchaintips has weird behaviour: first 2 lines of string, then the actual JSON array.
// todo: find out how to ignore stuff in serde.

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Blockcount(pub u32);

#[derive(Debug, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u32,
    pub headers: u32,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub verificationprogress: f64,
    pub chainwork: String,
    pub pruned: bool,
    pub commitments: u32,
    pub valuePools: Vec<ValuePool>,
    pub softforks: Vec<Softfork>,
    pub upgrades: Option<HashMap<String, Upgrade>>,
    pub consensus: Consensus,
}

#[derive(Debug, Deserialize)]
pub struct Consensus {
    pub chaintip: String,
    pub nextblock: String,
}

#[derive(Debug, Deserialize)]
pub struct Upgrade {
    pub name: String,
    pub activationheight: u32,
    pub status: String,
    pub info: String
}

#[derive(Debug, Deserialize)]
pub struct Softfork {
    pub id: String,
    pub version: u32,
    pub enforce: Enforce, // todo: could also be 1: SoftforkRule
    pub reject: Reject
}

#[derive(Debug, Deserialize)]
pub struct Enforce {
    pub status: bool,
    pub found: u32,
    pub required: u32,
    pub window: u32
}

#[derive(Debug, Deserialize)]
pub struct Reject {
    pub status: bool,
    pub found: u32,
    pub required: u32,
    pub window: u32
}

#[derive(Debug, Deserialize)]
pub struct ValuePool {
    pub id: String,
    pub monitored: bool,
    pub chainValue: f32,
    pub chainValueZat: u64,
}

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

#[derive(Debug, Deserialize)]
pub struct  Coinsupply {
    pub result: String,
    pub coin: String,
    pub height: u32,
    pub supply: f64,
    pub zfunds: f64,
    pub total: f64,
}