use base64;
use jsonrpc_client::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    ClientError, HTTPClient, JsonRpcVersion, RpcClient, RpcError, RpcRequest,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
//use rpc::BlockHash;
//use rpc::Transaction;
//use rpc::Info;
//use rpc::ChainTips;
use rpc::*;
use TransactionId;
use KomodoRpcApi;

use arguments::AddressList;

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

        // todo: show helpful error when credentials are false

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create http client");

        let rpc_client = RpcClient::new(client, url);

        KomodoClient {
            client: rpc_client
        }
    }

    fn send<R: DeserializeOwned + Debug, P: Serialize + Debug>(
        &self,
        request: &RpcRequest<P>
    ) -> Result<Result<R, RpcError>, ClientError> {
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

impl KomodoRpcApi for KomodoClient {
    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "gettransaction",
            tx,
        ))
    }

    fn get_info(
        &self,
    ) -> Result<Result<Info, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "curltest",
            "getinfo"
        ))
    }

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getbestblockhash",
        ))
    }

    fn get_new_address(&self) -> Result<Result<String, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getnewaddress",
        ))
    }

    fn get_difficulty(&self) -> Result<Result<f64, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getdifficulty",
        ))
    }

    fn dump_privkey(&self, address: &str) -> Result<Result<String, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "dumpprivkey",
            address
        ))
    }

    fn get_snapshot_max(&self, n: u32) -> Result<Result<Snapshot, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getsnapshot",
            n
        ))
    }

    fn get_snapshot(&self) -> Result<Result<Snapshot, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getsnapshot"
        ))
    }

    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<Result<AddressBalance, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressbalance",
            addresses
        ))
    }

    fn get_address_deltas(&self, addresses: &AddressList) -> Result<Result<AddressDeltas, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressdeltas",
            addresses
        ))
    }

    fn get_address_mempool(&self, addresses: &AddressList) -> Result<Result<AddressMempool, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressmempool",
            addresses
        ))
    }

    fn get_address_tx_ids(&self, addresses: &AddressList) -> Result<Result<AddressTxIDs, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddresstxids",
            addresses
        ))
    }

    fn get_address_utxos(&self, addresses: &AddressList) -> Result<Result<AddressUtxos, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressutxos",
            addresses
        ))
    }
}