use std::fs;

use base64;
use error::ApiError;

use ClientError;
use HTTPClient;
use RpcClient;
use RpcError;
use RpcRequest;
use JsonRpcVersion;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use rpc::*;
use TransactionId;
use BlockHash;
use KomodoRpcApi;
use chains::Chain;
use dirs;
use os_info::Type as OSType;

use arguments::AddressList;
use arguments::CreateRawTransactionOutputs;
use arguments::CreateRawTransactionInputs;

use std::collections::HashMap;

use std::io::Error;
use std::path::PathBuf;

pub struct Client {
    client: RpcClient,
    config: Config,
}

impl Client {
    /// Constructs a new `Client` that talks to the Komodo main chain. It assumes Komodo has
    /// been installed, since it fetches the needed RPC authentication parameters from the config file.
    pub fn new_komodo_client() -> Result<Self, ApiError> {
        let config = Config::get_for(Chain::KMD)?;
        let rpc_client = Client::construct_rpc_client(&config);

        Ok(Client {
            client: rpc_client,
            config,
        })
    }

    pub fn new_assetchain_client(ac: Chain) -> Result<Self, ApiError> {
        let config = Config::get_for(ac)?;
        let rpc_client = Client::construct_rpc_client(&config);

        Ok(Client {
            client: rpc_client,
            config,
        })
    }

    fn construct_rpc_client(config: &Config) -> RpcClient {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", config.rpc_user, config.rpc_password))
            )).unwrap(),
        );

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create http client");

        RpcClient::new(client, &format!("http://127.0.0.1:{}", config.rpc_port))
    }

    fn send<R: DeserializeOwned + Debug, P: Serialize + Debug>(
        &self,
        request: &RpcRequest<P>
    ) -> Result<R, ApiError> {
//    ) -> Result<Result<R, RpcError>, ClientError> {
//        let result = self.client.send::<R, P>(request);

//        match result {
//            Ok(Err(ref rpc_error)) if rpc_error.code == -28 => {
//                println!("komodod is still booting, try again")
//            }
//            _ => return result
//        }
        self.client.send(request)
    }
}

struct Config {
    rpc_user: String,
    rpc_password: String,
    rpc_port: u16,
}

impl Config {
    pub fn get_for(chain: Chain) -> Result<Self, ApiError> {
        let mut config_path: PathBuf = PathBuf::new();

        // find location of configuration file:
        match os_info::get().os_type() {
            OSType::Ubuntu | OSType::Linux => {
                if let Some(mut path) = dirs::home_dir() {
                    path.push(".komodo");
                    config_path = path;
                } else {
                    return Err(ApiError::Other(String::from("no komodod installation found")))
                }
            },
            OSType::Macos | OSType::Windows => {
                // MacOS: /Users/Alice/Library/Application Support
                // Windows: C:\Users\Alice\AppData\Roaming
                if let Some(mut path) = dirs::data_dir() {
                    path.push("Komodo");
                    config_path = path;
                } else {
                    return Err(ApiError::Other(String::from("no komodod installation found")))
                }
            },
            _ => return Err(ApiError::Other(String::from("unknown operating system")))
        }

        // push the actual configuration file:
        match chain {
            Chain::KMD => {
                config_path.push("komodo.conf"); // conf name is lowercase
            },
            // assetchain configuration files live in their own directory:
            _ => {
                config_path.push(chain.to_string());
                config_path.push(format!("{}.conf", chain.to_string())); // conf name is capitalized
            }
        }

        let contents = fs::read_to_string(config_path.to_str().unwrap())?;

        let map: HashMap<String, String> = contents.as_str()
            .split('\n')
            .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
            .filter(|vec| vec.len() == 2)
            .map(|vec| (
                vec[0].to_string(),
                vec[1].to_string()
            ))
            .collect::<HashMap<String, String>>();

        let _rpc_user = map.get("rpcuser").ok_or(ApiError::Config(String::from("No rpcuser in config")))?;
        let _rpc_password = map.get("rpcpassword").ok_or(ApiError::Config(String::from("no rpcpassword in config file")))?;
        let _rpc_port =
            match chain {
                Chain::KMD => "7771", // todo: KMD doesn't put rpcport in conf file at install
                _ => map.get("rpcport").ok_or(ApiError::Config(String::from("no rpcport in config file")))?,
            };

        Ok(Config {
            rpc_user:       _rpc_user.to_owned(),
            rpc_password:   _rpc_password.to_owned(),
            rpc_port:       _rpc_port.parse::<u16>().unwrap()
        })
    }
}

impl KomodoRpcApi for Client {
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<AddressBalance, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressbalance",
            addresses
        ))
    }

    fn get_address_deltas(&self, addresses: &AddressList) -> Result<AddressDeltas, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressdeltas",
            addresses
        ))
    }

    fn get_address_mempool(&self, addresses: &AddressList) -> Result<AddressMempool, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressmempool",
            addresses
        ))
    }

    fn get_address_tx_ids(&self, addresses: &AddressList) -> Result<AddressTxIDs, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddresstxids",
            addresses
        ))
    }

    fn get_address_utxos(&self, addresses: &AddressList) -> Result<AddressUtxos, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressutxos",
            addresses
        ))
    }

    fn get_snapshot_max(&self, n: u32) -> Result<Snapshot, ApiError> {
        // parameter must be string:
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "getsnapshot",
            n
        ))
    }

    fn get_snapshot(&self) -> Result<Snapshot,ApiError> {
        self.send(&RpcRequest::new0(
            "getsnapshot"
        ))
    }

    fn coinsupply(&self, n: u32) -> Result<Coinsupply, ApiError> {
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "coinsupply",
            n
        ))
    }

    fn get_best_block_hash(&self) -> Result<BlockHash, ApiError> {
        self.send(&RpcRequest::new0(
            "getbestblockhash",
        ))
    }

    fn get_block(&self, hashorheight: String) -> Result<Block, ApiError> {
        self.send(&RpcRequest::new1(
            "getblock",
            hashorheight
        ))
    }

    fn get_blockchain_info(&self) -> Result<BlockchainInfo, ApiError> {
        self.send(&RpcRequest::new0(
            "getblockchaininfo"
        ))
    }

    fn get_block_count(&self) -> Result<Blockcount, ApiError> {
        self.send(&RpcRequest::new0(
            "getblockcount"
        ))
    }

    fn get_block_hash(&self, n: u32) -> Result<BlockHash, ApiError> {
        self.send(&RpcRequest::new1(
            "getblockhash",
            n
        ))
    }

    fn get_block_header(&self, hash: String) -> Result<BlockHeader, ApiError> {
        self.send(&RpcRequest::new1(
            "getblockheader",
            hash
        ))
    }

    fn get_chaintips(&self) -> Result<ChainTips, ApiError> {
        self.send(&RpcRequest::new0(
            "getchaintips"
        ))
    }

    fn get_difficulty(&self) -> Result<f64, ApiError> {
        self.send(&RpcRequest::new0(
            "getdifficulty",
        ))
    }

    fn get_mempool_info(&self) -> Result<MempoolInfo, ApiError> {
        self.send(&RpcRequest::new0(
            "getmempoolinfo"
        ))
    }

    fn get_raw_mempool(&self) -> Result<RawMempool, ApiError> {
        self.send(&RpcRequest::new0(
            "getrawmempool"
        ))
    }

    fn get_raw_mempool_verbose(&self) -> Result<RawMempoolVerbose, ApiError> {
        self.send(&RpcRequest::new1(
            "getrawmempool",
            true
        ))
    }

    fn get_tx_out(&self, txid: String, index: u8) -> Result<TxOut, ApiError> {
        self.send(&RpcRequest::new2(
            "gettxout",
            txid,
            index
        ))
    }

    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo, ApiError> {
        self.send(&RpcRequest::new0(
            "gettxoutsetinfo"
        ))
    }

    fn minerids(&self, height: String) -> Result<MinerIDs, ApiError> { // why is height a string?
        self.send(&RpcRequest::new1(
            "minerids",
            height
        ))
    }

    fn notaries(&self, height: String) -> Result<Notaries, ApiError> { // why is height a string?
        self.send(&RpcRequest::new1(
            "notaries",
            height
        ))
    }

    fn get_info(&self) -> Result<Info, ApiError> {
        self.send(&RpcRequest::new0(
            "getinfo"
        ))
    }

    fn decode_raw_transaction(&self, hexstring: &str) -> Result<RawTransaction, ApiError> {
        self.send(&RpcRequest::new1(
            "decoderawtransaction",
            hexstring
        ))
    }

    fn decode_script(&self, hexstring: &str) -> Result<DecodedScript, ApiError> {
        self.send(&RpcRequest::new1(
            "decodescript",
            hexstring
        ))
    }

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<SerializedRawTransaction, ApiError> {
        self.send(&RpcRequest::new1(
            "getrawtransaction",
            txid
        ))
    }

    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<RawTransaction, ApiError> {
        self.send(&RpcRequest::new2(
            "getrawtransaction",
            txid,
            1
        ))
    }

    fn create_raw_transaction(&self, inputs: CreateRawTransactionInputs, outputs: CreateRawTransactionOutputs) -> Result<SerializedRawTransaction, ApiError> {
        self.send(&RpcRequest::new2(
            "createrawtransaction",
            inputs,
            outputs
        ))
    }

    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<SignedRawTransaction, ApiError> {
        self.send(&RpcRequest::new1(
            "signrawtransaction",
            hexstring
        ))
    }

    fn sign_raw_transaction_with_key(
        &self,
        hexstring: &SerializedRawTransaction,
        txoutput_detail: Option<Vec<&TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>
    ) -> Result<SignedRawTransaction, ApiError> {
        self.send(&RpcRequest::new4(
            "signrawtransaction",
            hexstring,
            txoutput_detail,
            private_keys,
            signature_hash_type
        ))
    }

    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<TransactionId, ApiError> {
        self.send(&RpcRequest::new1(
            "sendrawtransaction",
            &signed_tx.hex
        ))
    }

    fn backup_wallet(&self, file_name: &str) -> Result<String, ApiError> {
        self.send(&RpcRequest::new1(
            "backupwallet",
            file_name
        ))
    }

    fn dump_privkey(&self, address: &str) -> Result<String, ApiError> {
        self.send(&RpcRequest::new1(
            "dumpprivkey",
            address
        ))
    }

    fn dump_wallet(&self, filename: &str) -> Result<String, ApiError> {
        self.send(&RpcRequest::new1(
            "dumpwallet",
            filename
        ))
    }

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<f64, ApiError> {
        let mut second;
        let mut third;

        match (minconf, include_watchonly) {
            (Some(minconf), Some(wo)) => {
                second = minconf;
                third = wo;
            },
            (Some(minconf), _) => {
                second = minconf;
                third = false;
            },
            (_, Some(wo)) => {
                second = 1;
                third = wo;
            },
            _ => {
                second = 1;
                third = false;
            }
        }
        self.send(&RpcRequest::new3(
            "getbalance",
            "*",
            second,
            third
        ))
    }

    fn get_new_address(&self) -> Result<String, ApiError> {
        self.send(&RpcRequest::new0(
            "getnewaddress",
        ))
    }

    fn get_raw_change_address(&self) -> Result<String, ApiError> {
        self.send(&RpcRequest::new0(
            "getrawchangeaddress",
        ))
    }

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Transaction, ApiError> {
        self.send(&RpcRequest::new1(
            "gettransaction",
            tx,
        ))
    }

    fn get_wallet_info(&self) -> Result<WalletInfo, ApiError> {
        self.send(&RpcRequest::new0(
            "getwalletinfo"
        ))
    }

    fn get_block_subsidy(&self, height: Option<u64>) -> Result<BlockSubsidy, ApiError> {
        match height {
            Some(n) => {
                self.send(&RpcRequest::new1(
                    "getblocksubsidy",
                    height
                ))
            },
            None => {
                self.send(&RpcRequest::new0(
                    "getblocksubsidy",
                ))
            }
        }
    }

    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<BlockTemplate, ApiError> {
        match jsonrequestobject {
            Some(request) => {
                self.send(&RpcRequest::new1(
                    "getblocktemplate",
                    request
                ))
            },
            None => {
                self.send(&RpcRequest::new0(
                    "getblocktemplate",
                ))
            }
        }    }
}