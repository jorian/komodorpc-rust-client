#[derive(Deserialize, Serialize, Debug)]
pub enum AddNodeCommand {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "onetry")]
    Onetry
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddedNodeInfo {
    pub addednode: String, // is an IP
    pub connected: Option<bool>,
    pub addresses: Option<Vec<ConnectedAddress>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectedAddress {
    pub address: String, //is an IP
    pub connected: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeprecationInfo {
    pub version: u64,
    pub subversion: String,
    pub deprecationheight: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetTotals {
    pub totalbytesrecv: u64,
    pub totalbytessent: u64,
    pub timemillis: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetworkInfo {
    pub version: u64,
    pub subversion: String,
    pub protocolversion: u64,
    pub localservices: String,
    pub timeoffset: i32,
    pub connections: u32,
    pub networks: Vec<NetworkVersion>,
    pub relayfee: f64,
    pub localaddresses: Vec<Option<LocalAddress>>,
    pub warnings: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetworkVersion {
    pub name: String,
    pub limited: bool,
    pub reachable: bool,
    pub proxy: String,
    pub proxy_randomize_credentials: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocalAddress {
    pub address: String,
    pub port: u16,
    pub score: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Peer {
    pub id: u32,
    pub addr: String,
    pub addrlocal: String,
    pub services: String,
    pub lastsend: u64,
    pub lastrecv: u64,
    pub bytessent: u64,
    pub bytesrecv: u64,
    pub conntime: u64,
    pub timeoffset: i32,
    pub pingtime: f64,
    pub version: u32,
    pub subver: String,
    pub inbound: bool,
    pub startingheight: u64,
    pub banscore: u8,
    pub synced_headers: u64,
    pub synced_blocks: u64,
    pub inflight: Vec<Option<u64>>,
    pub whitelisted: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BannedNode {
    pub address: String,
    pub banned_until: u64
}