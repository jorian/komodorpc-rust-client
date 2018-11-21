use reqwest;
use serde_json;

pub struct RpcClient {
    client: reqwest::Client,
    url: String,
}

pub enum Error {
    Transport(reqwest::Error),
    Json(serde_json::Error),
}