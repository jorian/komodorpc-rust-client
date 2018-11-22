use reqwest;
use reqwest::Client as HttpClient;

use serde_json;
use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io::Read;
use rpcconn::RpcRequest;
use rpcconn::RpcError;
use rpcconn::RpcResponse;

pub struct RpcClient {
    client: HttpClient,
    url: String,
}

#[derive(Debug)]
pub enum Error {
    Transport(reqwest::Error),
    Json(serde_json::Error),
}

impl RpcClient {
    pub fn new(client: HttpClient, url: &str) -> Self {
        RpcClient {
            client,
            url: url.to_string(),
        }
    }

    pub fn send<R: Debug, T: Debug>(
        &self,
        request: &RpcRequest<T>,
    ) -> Result<Result<R, RpcError>, Error>
        where
            T: Serialize,
            R: DeserializeOwned,
    {

        let res = self
            .client
            .post(self.url.as_str())
            // TODO: Avoid serializing twice
            .json(request)
            .send()
            .map_err(Error::Transport)
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);
                serde_json::from_str(&buf).map_err(Error::Json)
            });

        res.map(RpcResponse::into_result)

        // TODO: Maybe check if req.id == res.id. Should always hold since it is a synchronous call.
    }
}