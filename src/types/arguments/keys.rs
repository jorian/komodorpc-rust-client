use ApiError;

#[derive(Deserialize, Serialize, Debug)]
pub struct ZKey(String);

impl ZKey {
    #[allow(dead_code)]
    pub fn from(key: &str) -> Result<ZKey, ApiError> {
        // todo validate zkey
        Ok(ZKey { 0: key.to_string() })
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Rescan {
    Yes,
    No,
    WhenKeyIsNew,
}