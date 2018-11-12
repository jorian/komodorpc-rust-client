

// getchaintips has weird behaviour: first 2 lines of string, then the actual JSON array.
// todo: find out how to ignore stuff in serde.

use std::collections::HashMap;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

#[derive(Debug, Deserialize)]
pub struct BlockHeader {
    pub hash: String,
    pub confirmations: u32,
    pub height: u32,
    pub version: u32,
    pub merkleroot: String,
    pub time: u32,
    pub nonce: String,
    pub solution: String,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub segid: i32,
    pub previousblockhash: Option<String>, // oldest block has no previous block
    pub nextblockhash: Option<String>, // newest block has no next block
}

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
pub struct ChainTips(Vec<ChainTip>);

#[derive(Debug, Deserialize)]
pub struct ChainTip {
    pub height: u64,
    pub hash: String,
    pub branchlen: u32,
    pub status: ChainTipStatus
}

#[derive(Debug)]
pub enum ChainTipStatus {
    Invalid,
    HeadersOnly,
    ValidHeaders,
    ValidFork,
    Active
}

impl<'de> Deserialize<'de> for ChainTipStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let chaintipstatus = match s.as_str() {
            "active"                    => ChainTipStatus::Active,
            "invalid"                   => ChainTipStatus::Invalid,
            "headers-only"              => ChainTipStatus::HeadersOnly,
            "valid-headers"             => ChainTipStatus::ValidHeaders,
            "valid-fork"                => ChainTipStatus::ValidFork,
            other => { return Err(de::Error::custom(format!("Invalid state '{}'", other))); },
        };
        Ok(chaintipstatus)
    }
}

#[derive(Debug, Deserialize)]
pub struct Coinsupply {
    pub result: String,
    pub coin: String,
    pub height: u32,
    pub supply: f64,
    pub zfunds: f64,
    pub total: f64,
}

#[derive(Debug, Deserialize)]
pub struct MempoolInfo {
    pub size: u32,
    pub bytes: u32,
    pub usage: u32,
}

#[derive(Debug, Deserialize)]
pub struct RawMempool(Vec<String>);

#[derive(Debug, Deserialize)]
pub struct RawMempoolVerbose(HashMap<String, RawMempoolTransactionInfo>);

#[derive(Debug, Deserialize)]
pub struct RawMempoolTransactionInfo {
    pub size: u32,
    pub fee: f32,
    pub time: u32,
    pub height: u32,
    pub startingpriority: f64,
    pub currentpriority: f64,
    pub depends: Vec<String>, // this either returns an empty array or an array with txids
}