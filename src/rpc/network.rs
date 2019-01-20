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