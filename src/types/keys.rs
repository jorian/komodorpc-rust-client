#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateKey(pub String);

#[derive(Deserialize, Debug)]
pub struct ViewingKey(pub String);