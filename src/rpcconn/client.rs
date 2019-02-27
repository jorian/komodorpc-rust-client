use reqwest;
use reqwest::Client as HttpClient;

use serde_json;
use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io::Read;
use rpcconn::RpcRequest;
use rpcconn::RpcResponse;

use std::error;
use std::fmt;

use error::ApiError;
use std::error::Error;

pub struct RpcClient {
    client: HttpClient,
    url: String,
}

#[derive(Debug)]
pub enum ClientError {
    Transport(reqwest::Error),
    Json(serde_json::Error),
}

impl error::Error for ClientError {
    //noinspection RsUnresolvedReference
    fn description(&self) -> &str {
        match self {
            ClientError::Transport(ref e) => e.description(),
            ClientError::Json(ref e) => e.description(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something bad happened: {}", self.description())
    }
}

impl RpcClient {
    pub fn new(client: HttpClient, url: &str) -> Self {
        RpcClient {
            client,
            url: url.to_string(),
        }
    }

    pub fn send<R, T>(
        &self,
        request: &RpcRequest<T>,
    ) -> Result<R, ApiError>
        where
            T: Serialize + Debug,
            R: DeserializeOwned + Debug,
    {
        let res = self
            .client
            .post(self.url.as_str())
            // TODO: Avoid serializing twice
            .json(request)
            .send()
            .map_err(|err| ClientError::Transport(err))
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);

                serde_json::from_str(&buf).map_err(|err| ClientError::Json(err))
            });

        let res = res.map(RpcResponse::into_result);

        match res {
            Ok(one) => {
                match one {
                    Ok(two) => Ok(two),
                    Err(rpc_error) => Err(ApiError::RPC(rpc_error))
                }
            },
            Err(client_error) => Err(ApiError::Client(client_error)),
        }

        // TODO: Maybe check if req.id == res.id. Should always hold since it is a synchronous call.
    }
}