use ApiError;

#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateKey(pub String);

impl PrivateKey {
    pub fn from_string(key: String) -> Result<PrivateKey, ApiError> {
        if key.len() != 52 {
            return Err(ApiError::Other(format!("Private key has incorrect length, needs to be valid 52-char WIF")));
        }
        Ok(PrivateKey {
            0: key.clone()
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct ViewingKey(pub String);