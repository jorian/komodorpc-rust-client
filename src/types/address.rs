use serde::de::{self, Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt::Display;
use types::arguments::address::Address;

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

#[derive(Debug, Deserialize, Clone)]
pub struct AddressUtxos(pub Vec<AddressUtxo>);

#[derive(Debug, Deserialize, Clone)]
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
    pub ending_height: u64,
    pub end_time: u64,
    pub ignored_addresses: u32,
    pub skipped_cc_utxos: u32,
    pub cc_utxo_value: u32,
    #[serde(rename = "total_includeCCvouts")]
    pub total_include_ccvouts: f64,

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

#[derive(Debug, Deserialize)]
pub struct AddressGroupings(Vec<Vec<Vec<AddressGrouping>>>);

#[derive(Debug, Deserialize)]
pub struct AddressGrouping {
    address: Address,
    amount: f64,
    account: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Multisig {
    pub address: Address,
    #[serde(rename = "redeemScript")]
    redeem_script: String
}

//impl<'de> Deserialize<'de> for AddressGrouping {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
//        D: Deserializer<'de> {
//        let s: &str = Deserialize::deserialize(deserializer)?;
//        dbg!(&s);
//
//        Ok(AddressGrouping {
//            address: Address::from("RKakNo1Vz86xiCLB5vq5UBe4P7sB9Gw6pv").unwrap(),
//            amount: 10.0,
//            account: None,
//        })
//
//    }
//}