#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Info {
    pub version: u32,
    pub protocolversion: u32,
    #[serde(rename = "KMDversion")]
    pub kmdversion: String,
    pub notarized: u32,
    #[serde(rename = "prevMoMheight")]
    pub prev_mom_height: u32,
    pub notarizedhash: String,
    pub notarizedtxid: String,
    pub notarizedtxid_height: String,
    pub notarized_confirms: u32,
    pub walletversion: u32,
    pub balance: f32,
    pub interest: Option<f32>,
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
    pub sapling: u64,
    pub p2pport: u32,
    pub rpcport: u32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct WalletInfo {
    pub walletversion: u32,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub immature_balance: f64,
    pub txcount: u32,
    pub keypoololdest: u64,
    pub keypoolsize: u32,
    pub paytxfee: f32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TxOutSetInfo {
    pub height: u32,
    pub bestblock: String,
    pub transactions: u32,
    pub txouts: u32,
    pub bytes_serialized: u32,
    pub hash_serialized: String,
    pub total_amount: f64,
}