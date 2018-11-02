use serde::{de, Deserialize, Deserializer, Serialize, Serializer};


#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Info {
    pub version: u32,
    pub protocolversion: u32,
    pub KMDversion: String,
    pub notarized: u32,
    pub prevMoMheight: u32,
    pub notarizedhash: String,
    pub notarizedtxid: String,
    pub notarizedtxid_height: String,
    pub notarized_confirms: u32,
    pub walletversion: u32,
    pub balance: f32,
    pub interest: f32,
    pub blocks: u32,
    pub longestchain: u32,
    pub timeoffset: u32,
    pub tiptime: u32,
    pub connections: u32,
    pub proxy: String,
    pub difficulty: f64,
    pub testnet: bool,
    pub keypoololdest: u32,
    pub keypoolsize: u32,
    pub paytxfee: f32,
    pub relayfee: f32,
    pub errors: String,
    pub name: String,
}