use serde::de::{self, Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct AddressBalance {
    pub balance: u64,
    pub received: u64,
}

#[derive(Debug, Deserialize)]
pub struct AddressDeltas(pub Vec<AddressDelta>);


#[derive(Debug, Deserialize)]
pub struct AddressDelta {
    pub satoshis: i64,
    pub txid: String,
    pub index: u32,
    pub blockindex: u64,
    pub height: u64,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct AddressMempool(pub Vec<AddressMempoolDelta>);

#[derive(Debug, Deserialize)]
pub struct AddressMempoolDelta {
    address: String,
    txid: String,
    index: u32,
    satoshis: i64,
    timestamp: u64,
    prevtxid: String,
    prevout: u32,
}

#[derive(Debug, Deserialize)]
pub struct AddressTxIDs(pub Vec<String>); // todo: a vec with txids, needs work.

#[derive(Debug, Deserialize)]
pub struct AddressUtxos(pub Vec<AddressUtxo>);

#[derive(Debug, Deserialize)]
pub struct AddressUtxo {
    pub address: String,
    pub txid: String,
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    pub script: String,
    pub satoshis: u64, // output always positive, no signing needed
    pub height: u64,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub start_time: u64,
    pub addresses: Vec<SnapshotAddress>,
    pub total: f64,
    pub average: f64,
    pub utxos: u64,
    pub total_addresses: u64,
    pub start_height: u64,
    pub ending_height: u64,
    pub end_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotAddress {
    pub addr: String,
    #[serde(deserialize_with = "from_str")]
    pub amount: f64
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}