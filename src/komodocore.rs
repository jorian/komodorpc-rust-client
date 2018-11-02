use base64;
use jsonrpc_client::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    ClientError, HTTPClient, JsonRpcVersion, RpcClient, RpcError, RpcRequest,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use types::BlockHash;

pub struct KomodoClient {
    client: RpcClient
}

#[allow(dead_code)]
impl KomodoClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", username, password))
            )).unwrap(),
        );

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create http client");

        let rpc_client = RpcClient::new(client, url);

        KomodoClient {
            client: rpc_client
        }
    }

    pub fn get_info<R: Debug>(
        &self,
    ) -> Result<Result<R, RpcError>, ClientError>
        where
            R: DeserializeOwned
    {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "curltest",
            "getinfo"
        ))
    }

    pub fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getbestblockhash",
        ))
    }

    fn send<R: DeserializeOwned + Debug, P: Serialize + Debug>(&self, request: &RpcRequest<P>) -> Result<Result<R, RpcError>, ClientError> {
        let result = self.client.send::<R, P>(request);

        match result {
            Ok(Err(ref rpc_error)) if rpc_error.code == -28 => {
                println!("komodod is still booting, try again")
            }
            _ => return result
        }
        self.client.send(request)
    }
}