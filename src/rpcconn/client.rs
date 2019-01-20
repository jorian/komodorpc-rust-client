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
        write!(f, "Something bad happened!")
    }
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
    ) -> Result<R, ApiError>
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
            .map_err(|err| ClientError::Transport(err))
            .and_then(|mut res| {
                let mut buf = String::new();

                // this basically reads the response of the request into the String `buf`
                let _ = res.read_to_string(&mut buf);

                // then, the String buf is read into serde to be made a JSON:
                serde_json::from_str(&buf).map_err(|err| ClientError::Json(err))
            });

        // give the response meaning: return an Err when the Komodod response is an error,
        // and return an Ok if the response contains a result
        let res = res.map(RpcResponse::into_result);

        println!("{:#?}", res);

        match res {
            Ok(result) => {
                match result {
                    Err(e) => Err(ApiError::RPC(e)),
                    Ok(res2) => {
//                        println!("{:?}", res2);
                        Ok(res2)
                    }
                }
            },
            Err(e) => Err(ApiError::Client(e))
        }
        // here is a result from the request with an id,
        // optionally the result (whatever it is) and
        // optionally an error. this is now morphed into an actual Result, where if there is an error
        // coming from komodod, the RpcResponse is an RpcError.

//        res.map(RpcResponse::into_result) // Result<T(he response), RpcError>

        // TODO: Maybe check if req.id == res.id. Should always hold since it is a synchronous call.
    }
}