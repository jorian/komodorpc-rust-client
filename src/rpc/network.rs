#[derive(Deserialize, Serialize, Debug)]
pub enum AddNodeCommand {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "onetry")]
    Onetry
}