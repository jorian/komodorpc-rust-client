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
    pub transparent: f64,
    #[serde(deserialize_with = "from_str")]
    pub interest: f64,
    #[serde(deserialize_with = "from_str")]
    pub private: f64,
    #[serde(deserialize_with = "from_str")]
    pub total: f64,
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
    pub id: String,
    pub status: String, // failed, cancelled or success
    pub creation_time: u64,
    pub result: HashMap<String, String>,
    pub execution_secs: f64,
    pub method: String,
    pub params: OperationParams,
}

#[derive(Deserialize, Debug)]
pub struct OperationParams {
    pub fromaddress: Address,
    pub amounts: Vec<Amount>,
    pub minconf: u32,
    pub fee: f64
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    pub address: Address,
    pub amount: f64
}

#[derive(Deserialize, Debug)]
pub struct ReceivedByAddress(pub Vec<Received>);

#[derive(Deserialize, Debug)]
pub struct Received {
    pub txid: TransactionId,
    pub amount: f64,
    pub memo: String,
    pub outindex: u32,
    pub rawconfirmations: u32,
    pub confirmations: u32,
    pub change: bool
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