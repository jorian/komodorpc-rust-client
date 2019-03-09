use std::collections::HashMap;
use arguments::address::Address;
use serde::de::{self, Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt::Display;
use TransactionId;

#[derive(Deserialize, Debug)]
pub struct Balance(pub f64);

#[derive(Debug, Deserialize)]
pub struct TotalBalance {
    #[serde(deserialize_with = "from_str")]
    transparent: f64,
    #[serde(deserialize_with = "from_str")]
    interest: f64,
    #[serde(deserialize_with = "from_str")]
    private: f64,
    #[serde(deserialize_with = "from_str")]
    total: f64,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct Operations(pub Vec<Operation>);

#[derive(Deserialize, Debug)]
pub struct Operation {
    id: String,
    status: String, // failed, cancelled or success
    creation_time: u64,
    result: HashMap<String, String>,
    execution_secs: f64,
    method: String,
    params: OperationParams,
}

#[derive(Deserialize, Debug)]
pub struct OperationParams {
    fromaddress: Address,
    amounts: Vec<Amount>,
    minconf: u32,
    fee: f64
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    address: Address,
    amount: f64
}

#[derive(Deserialize, Debug)]
pub struct ReceivedByAddress(pub Vec<Received>);

#[derive(Deserialize, Debug)]
pub struct Received {
    txid: TransactionId,
    amount: f64,
    memo: String,
    outindex: u32,
    rawconfirmations: u32,
    confirmations: u32,
    change: bool
}

#[derive(Deserialize, Debug)]
pub struct MergeResult {
    #[serde(rename = "remainingUTXOs")]
    pub remaining_utxos: u32,
    #[serde(rename = "remainingTransparentValue")]
    pub remaining_transparent_value: f64,
    #[serde(rename = "remainingNotes")]
    pub remaining_notes: u32,
    #[serde(rename = "remainingShieldedValue")]
    pub remaining_shielded_value: f64,
    #[serde(rename = "mergingUTXOs")]
    pub merging_utxos: u32,
    #[serde(rename = "mergingTransparentValue")]
    pub merging_transparent_value: f64,
    #[serde(rename = "mergingNotes")]
    pub merging_notes: u32,
    #[serde(rename = "mergingShieldedValue")]
    pub merging_shielded_value: f64,
    pub opid: String,
}

#[derive(Deserialize, Debug)]
pub struct ShieldResult {
    #[serde(rename = "remainingUTXOs")]
    pub remaining_utxos: u32,
    #[serde(rename = "remainingValue")]
    pub remaining_value: u32,
    #[serde(rename = "shieldingUTXOs")]
    pub shielding_utxos: u32,
    #[serde(rename = "shieldingValue")]
    pub shielding_value: u32,
    pub opid: String
}