use std::fs;

use base64;
use error::ApiError;

use HTTPClient;
use RpcClient;
use RpcRequest;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use types::*;
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

use std::path::PathBuf;

pub struct Client {
    client: RpcClient,
}

impl Client {
    /// Constructs a new `Client` that talks to the Komodo main chain. It assumes Komodo has
    /// been installed, since it fetches the needed RPC authentication parameters from the config file.
    pub fn new_komodo_client() -> Result<Self, ApiError> {
        let config = Config::get_for(Chain::KMD)?;
        let rpc_client = Client::construct_rpc_client(&config);

        Ok(Client {
            client: rpc_client
        })
    }

    pub fn new_assetchain_client(ac: Chain) -> Result<Self, ApiError> {
        let config = Config::get_for(ac)?;
        let rpc_client = Client::construct_rpc_client(&config);

        Ok(Client {
            client: rpc_client,
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
    ) -> Result<Option<R>, ApiError> {
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
        let mut config_path: PathBuf;

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
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<Option<AddressBalance>, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressbalance",
            addresses
        ))
    }

    fn get_address_deltas(&self, addresses: &AddressList) -> Result<Option<AddressDeltas>, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressdeltas",
            addresses
        ))
    }

    fn get_address_mempool(&self, addresses: &AddressList) -> Result<Option<AddressMempool>, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressmempool",
            addresses
        ))
    }

    fn get_address_tx_ids(&self, addresses: &AddressList) -> Result<Option<AddressTxIDs>, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddresstxids",
            addresses
        ))
    }

    fn get_address_utxos(&self, addresses: &AddressList) -> Result<Option<AddressUtxos>, ApiError> {
        self.send(&RpcRequest::new1(
            "getaddressutxos",
            addresses
        ))
    }

    fn get_snapshot_max(&self, n: u32) -> Result<Option<Snapshot>, ApiError> {
        // parameter must be string:
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "getsnapshot",
            n
        ))
    }

    fn get_snapshot(&self) -> Result<Option<Snapshot>,ApiError> {
        self.send(&RpcRequest::new0(
            "getsnapshot"
        ))
    }

    fn coinsupply(&self, n: u32) -> Result<Option<Coinsupply>, ApiError> {
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "coinsupply",
            n
        ))
    }

    fn get_best_block_hash(&self) -> Result<Option<BlockHash>, ApiError> {
        self.send(&RpcRequest::new0(
            "getbestblockhash",
        ))
    }

    fn get_block(&self, hashorheight: String) -> Result<Option<Block>, ApiError> {
        self.send(&RpcRequest::new1(
            "getblock",
            hashorheight
        ))
    }

    fn get_blockchain_info(&self) -> Result<Option<BlockchainInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getblockchaininfo"
        ))
    }

    fn get_block_count(&self) -> Result<Option<Blockcount>, ApiError> {
        self.send(&RpcRequest::new0(
            "getblockcount"
        ))
    }

    fn get_block_hash(&self, n: u32) -> Result<Option<BlockHash>, ApiError> {
        self.send(&RpcRequest::new1(
            "getblockhash",
            n
        ))
    }

    fn get_block_header(&self, hash: String) -> Result<Option<BlockHeader>, ApiError> {
        self.send(&RpcRequest::new1(
            "getblockheader",
            hash
        ))
    }

    fn get_chaintips(&self) -> Result<Option<ChainTips>, ApiError> {
        self.send(&RpcRequest::new0(
            "getchaintips"
        ))
    }

    fn get_difficulty(&self) -> Result<Option<f64>, ApiError> {
        self.send(&RpcRequest::new0(
            "getdifficulty",
        ))
    }

    fn get_mempool_info(&self) -> Result<Option<MempoolInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getmempoolinfo"
        ))
    }

    fn get_raw_mempool(&self) -> Result<Option<RawMempool>, ApiError> {
        self.send(&RpcRequest::new0(
            "getrawmempool"
        ))
    }

    fn get_raw_mempool_verbose(&self) -> Result<Option<RawMempoolVerbose>, ApiError> {
        self.send(&RpcRequest::new1(
            "getrawmempool",
            true
        ))
    }

    fn get_tx_out(&self, txid: String, index: u8) -> Result<Option<TxOut>, ApiError> {
        self.send(&RpcRequest::new2(
            "gettxout",
            txid,
            index
        ))
    }

    fn get_tx_out_set_info(&self) -> Result<Option<TxOutSetInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "gettxoutsetinfo"
        ))
    }

    fn minerids(&self, height: String) -> Result<Option<MinerIDs>, ApiError> { // why is height a string?
        self.send(&RpcRequest::new1(
            "minerids",
            height
        ))
    }

    fn notaries(&self, height: String) -> Result<Option<Notaries>, ApiError> { // why is height a string?
        self.send(&RpcRequest::new1(
            "notaries",
            height
        ))
    }

    fn get_info(&self) -> Result<Option<Info>, ApiError> {
        self.send(&RpcRequest::new0(
            "getinfo"
        ))
    }

    fn get_block_subsidy(&self, height: Option<u64>) -> Result<Option<BlockSubsidy>, ApiError> {
        match height {
            Some(n) => {
                self.send(&RpcRequest::new1(
                    "getblocksubsidy",
                    n
                ))
            },
            None => {
                self.send(&RpcRequest::new0(
                    "getblocksubsidy",
                ))
            }
        }
    }

    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<Option<BlockTemplate>, ApiError> {
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

    fn get_local_sol_ps(&self) -> Result<Option<f64>, ApiError> {
        self.send(&RpcRequest::new0(
            "getlocalsolps"
        ))
    }

    fn get_mining_info(&self) -> Result<Option<MiningInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getmininginfo"
        ))
    }

    fn get_network_hash_ps(&self) -> Result<Option<u64>, ApiError> {
        self.send(&RpcRequest::new0(
            "getnetworkhashps"
        ))
    }

    fn get_network_sol_ps(&self) -> Result<Option<u64>, ApiError> {
        self.send(&RpcRequest::new0(
            "getnetworksolps"
        ))
    }

    // todo untested
    fn prioritise_transaction(&self, txid: TransactionId, prio_delta: f64, fee_delta: u32) -> Result<Option<bool>, ApiError> {
        self.send(&RpcRequest::new3(
            "prioritisetransaction",
            txid,
            prio_delta,
            fee_delta
        ))
    }

    // todo untested
    fn submit_block(&self, hexdata: String, jsonparametersobject: Option<ParametersObject>) -> Result<Option<SubmitBlockResult>, ApiError> {
        match jsonparametersobject {
            Some(object) => {
                self.send(&RpcRequest::new2(
                    "submitblock",
                    hexdata,
                    object
                ))
            },
            None => {
                self.send(&RpcRequest::new1(
                    "submitblock",
                    hexdata
                ))
            }
        }
    }

    fn addnode(&self, node: String, action: AddNodeCommand) -> Result<Option<()>, ApiError> {
        self.send(&RpcRequest::new2(
            "addnode",
            node,
            action
        ))
    }

    fn clear_banned(&self) -> Result<Option<()>, ApiError> {
        self.send(&RpcRequest::new0(
            "clearbanned",
        ))
    }

    fn disconnect_node(&self, node: String) -> Result<Option<()>, ApiError> {
        self.send(&RpcRequest::new1(
            "addnode",
            node
        ))
    }

    fn get_added_node_info(&self, dns: bool, node: Option<&str>) -> Result<Option<Vec<AddedNodeInfo>>, ApiError> {
        match node {
            Some(address) => {
                self.send(&RpcRequest::new2(
                    "getaddednodeinfo",
                    dns,
                    address
                ))
            },
            None => {
                self.send(&RpcRequest::new1(
                    "getaddednodeinfo",
                    dns
                ))
            }
        }
    }

    fn get_connection_count(&self) -> Result<Option<u32>, ApiError> {
        self.send(&RpcRequest::new0(
            "getconnectioncount"
        ))
    }

    fn get_deprecation_info(&self) -> Result<Option<DeprecationInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getdeprecationinfo"
        ))
    }

    fn get_net_totals(&self) -> Result<Option<NetTotals>, ApiError> {
        self.send(&RpcRequest::new0(
            "getnettotals"
        ))
    }

    fn get_network_info(&self) -> Result<Option<NetworkInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getnetworkinfo"
        ))
    }

    fn get_peer_info(&self) -> Result<Option<Vec<Peer>>, ApiError> {
        self.send(&RpcRequest::new0(
            "getpeerinfo"
        ))
    }

    fn list_banned(&self) -> Result<Option<Vec<Option<BannedNode>>>, ApiError> {
        self.send(&RpcRequest::new0(
            "listbanned"
        ))
    }

    fn ping(&self) -> Result<Option<()>, ApiError> {
        self.send(&RpcRequest::new0(
            "ping"
        ))
    }

    fn set_ban(
        &self,
        ip: String,
        command: String,
        bantime: Option<u32>,
        absolute: Option<bool>
    ) -> Result<Option<()>, ApiError> {
        match bantime {
            Some(time) => {
                match absolute {
                    Some(true) => {
                        self.send(&RpcRequest::new4(
                            "setban",
                            ip,
                            command,
                            time,
                            true
                        ))
                    },
                    Some(false) => {
                        self.send(&RpcRequest::new4(
                            "setban",
                            ip,
                            command,
                            time,
                            false
                        ))
                    },
                    None => unreachable!()
                }
            },
            None => {
                self.send(&RpcRequest::new2(
                    "setban",
                    ip,
                    command,
                ))
            }
        }
    }

    fn create_raw_transaction(&self, inputs: CreateRawTransactionInputs, outputs: CreateRawTransactionOutputs) -> Result<Option<SerializedRawTransaction>, ApiError> {
        self.send(&RpcRequest::new2(
            "createrawtransaction",
            inputs,
            outputs
        ))
    }

    fn decode_raw_transaction(&self, hexstring: &str) -> Result<Option<RawTransaction>, ApiError> {
        self.send(&RpcRequest::new1(
            "decoderawtransaction",
            hexstring
        ))
    }

    fn decode_script(&self, hexstring: &str) -> Result<Option<DecodedScript>, ApiError> {
        self.send(&RpcRequest::new1(
            "decodescript",
            hexstring
        ))
    }

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<Option<SerializedRawTransaction>, ApiError> {
        self.send(&RpcRequest::new1(
            "getrawtransaction",
            txid
        ))
    }

    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<Option<RawTransaction>, ApiError> {
        self.send(&RpcRequest::new2(
            "getrawtransaction",
            txid,
            1
        ))
    }

    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<Option<SignedRawTransaction>, ApiError> {
        self.send(&RpcRequest::new1(
            "signrawtransaction",
            hexstring
        ))
    }

    fn sign_raw_transaction_with_key(
        &self,
        hexstring: &SerializedRawTransaction,
        txoutput_detail: Option<Vec<TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>
    ) -> Result<Option<SignedRawTransaction>, ApiError> {
        self.send(&RpcRequest::new4(
            "signrawtransaction",
            hexstring,
            txoutput_detail,
            private_keys,
            signature_hash_type
        ))
    }

    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<Option<TransactionId>, ApiError> {
        self.send(&RpcRequest::new1(
            "sendrawtransaction",
            &signed_tx.hex
        ))
    }

    fn backup_wallet(&self, file_name: &str) -> Result<Option<String>, ApiError> {
        self.send(&RpcRequest::new1(
            "backupwallet",
            file_name
        ))
    }

    fn dump_privkey(&self, address: &str) -> Result<Option<String>, ApiError> {
        self.send(&RpcRequest::new1(
            "dumpprivkey",
            address
        ))
    }

    fn dump_wallet(&self, filename: &str) -> Result<Option<String>, ApiError> {
        self.send(&RpcRequest::new1(
            "dumpwallet",
            filename
        ))
    }

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<Option<f64>, ApiError> {
        let second;
        let third;

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

    fn get_new_address(&self) -> Result<Option<String>, ApiError> {
        self.send(&RpcRequest::new0(
            "getnewaddress",
        ))
    }

    fn get_raw_change_address(&self) -> Result<Option<String>, ApiError> {
        self.send(&RpcRequest::new0(
            "getrawchangeaddress",
        ))
    }

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Option<Transaction>, ApiError> {
        self.send(&RpcRequest::new1(
            "gettransaction",
            tx,
        ))
    }

    fn get_wallet_info(&self) -> Result<Option<WalletInfo>, ApiError> {
        self.send(&RpcRequest::new0(
            "getwalletinfo"
        ))
    }
}