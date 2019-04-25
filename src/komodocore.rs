use std::fs;
use std::path::PathBuf;
use std::fmt::Debug;
use std::collections::HashMap;

use os_info::Type as OSType;
use base64;
use dirs;

use HTTPClient;
use RpcClient;
use RpcRequest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use serde::{de::DeserializeOwned, ser::Serialize};

use TransactionId;
use BlockHash;
use KomodoRpcApi;
use chains::Chain;

use arguments::*;
use types::*;

use error::ApiError;
use types::arguments::address::{Address, Amounts};
use arguments::address::{AddrType, FromAddresses};
use std::io::{Error as IOError, ErrorKind};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug)]
pub struct Client {
    client: RpcClient,
}

impl Client {
    /// Constructs a new `Client` that talks to the Komodo main chain. It assumes Komodo has
    /// been installed and run at least once, since it fetches the needed RPC authentication parameters
    /// from the config file which is created upon initialisation.
    pub fn new_komodo_client() -> Result<Client> {
        let config = Config::get_for(&Chain::KMD)?;
        let rpc_client = Client::construct_rpc_client(&config);

        Ok(Client {
            client: rpc_client
        })
    }

    /// Constructs a new `Client` that talks to the specified assetchain. It assumes Komodo has
    /// been installed and the assetchain has been started at least once, in order to be able to fetch
    /// the needed RPC authentication parameters from the assetchain config file.
    ///
    /// If `Custom(String)` is used as parameter, this function will throw a std::io::Error::NotFound if
    /// the defined config folder does not exist.
    pub fn new_assetchain_client(ac: &Chain) -> Result<Self> {
        let config = Config::get_for(&ac)?;
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
    ) -> Result<R> {
        dbg!(&request);
        self.client.send(request)
    }
}

struct Config {
    rpc_user: String,
    rpc_password: String,
    rpc_port: u16,
}

impl Config {
    pub fn get_for(chain: &Chain) -> Result<Self> {
        let mut config_path: PathBuf;

        // find location of configuration file:
        match os_info::get().os_type() {
            OSType::Ubuntu | OSType::Linux => {
                // Linux: /home/$USER/
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
            _ => return Err(ApiError::Other(String::from("unknown or unsupported operating system")))
        }

        // push the actual configuration file:
        match chain {
            Chain::KMD => {
                config_path.push("komodo.conf"); // conf name is lowercase
            },
            Chain::Custom(chain) => {
                config_path.push(chain);

                if !config_path.is_dir() {
                    return Err(ApiError::IO(IOError::from(ErrorKind::NotFound)));
                }

                config_path.push(format!("{}.conf", chain.to_string()));
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
                Chain::KMD => "7771", // KMD doesn't put rpcport in conf file at install
                _ => map.get("rpcport").ok_or(ApiError::Config(String::from("no rpcport in config file")))?,
            };

        Ok(Config {
            rpc_user:       _rpc_user.to_owned(),
            rpc_password:   _rpc_password.to_owned(),
            rpc_port:       _rpc_port.parse::<u16>()?
        })
    }
}

impl KomodoRpcApi for Client {
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<AddressBalance> {
        self.send(&RpcRequest::new1(
            "getaddressbalance",
            addresses
        ))
    }

    fn get_address_deltas(&self, addresses: &AddressList) -> Result<AddressDeltas> {
        self.send(&RpcRequest::new1(
            "getaddressdeltas",
            addresses
        ))
    }

    fn get_address_mempool(&self, addresses: &AddressList) -> Result<AddressMempool> {
        self.send(&RpcRequest::new1(
            "getaddressmempool",
            addresses
        ))
    }

    fn get_address_tx_ids(&self, addresses: &AddressList) -> Result<AddressTxIDs> {
        self.send(&RpcRequest::new1(
            "getaddresstxids",
            addresses
        ))
    }

    fn get_address_utxos(&self, addresses: &AddressList) -> Result<AddressUtxos> {
        self.send(&RpcRequest::new1(
            "getaddressutxos",
            addresses
        ))
    }

    fn get_snapshot_max(&self, n: u32) -> Result<Snapshot> {
        // parameter must be string:
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "getsnapshot",
            n
        ))
    }

    fn get_snapshot(&self) -> Result<Snapshot> {
        self.send(&RpcRequest::new0(
            "getsnapshot"
        ))
    }

    fn coinsupply(&self, n: u32) -> Result<Coinsupply> {
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            "coinsupply",
            n
        ))
    }

    fn get_best_block_hash(&self) -> Result<BlockHash> {
        self.send(&RpcRequest::new0(
            "getbestblockhash",
        ))
    }

    fn get_block(&self, hashorheight: String) -> Result<Block> {
        self.send(&RpcRequest::new1(
            "getblock",
            hashorheight
        ))
    }

    fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.send(&RpcRequest::new0(
            "getblockchaininfo"
        ))
    }

    fn get_block_count(&self) -> Result<Blockcount> {
        self.send(&RpcRequest::new0(
            "getblockcount"
        ))
    }

    fn get_block_hash(&self, n: u32) -> Result<BlockHash> {
        self.send(&RpcRequest::new1(
            "getblockhash",
            n
        ))
    }

    fn get_block_header(&self, hash: String) -> Result<BlockHeader> {
        self.send(&RpcRequest::new1(
            "getblockheader",
            hash
        ))
    }

    fn get_chaintips(&self) -> Result<ChainTips> {
        self.send(&RpcRequest::new0(
            "getchaintips"
        ))
    }

    fn get_difficulty(&self) -> Result<f64> {
        self.send(&RpcRequest::new0(
            "getdifficulty",
        ))
    }

    fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.send(&RpcRequest::new0(
            "getmempoolinfo"
        ))
    }

    fn get_raw_mempool(&self) -> Result<RawMempool> {
        self.send(&RpcRequest::new0(
            "getrawmempool"
        ))
    }

    fn get_raw_mempool_verbose(&self) -> Result<RawMempoolVerbose> {
        self.send(&RpcRequest::new1(
            "getrawmempool",
            true
        ))
    }

    fn get_tx_out(&self, txid: String, index: u8) -> Result<TxOut> {
        self.send(&RpcRequest::new2(
            "gettxout",
            txid,
            index
        ))
    }

    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo> {
        self.send(&RpcRequest::new0(
            "gettxoutsetinfo"
        ))
    }

    fn minerids(&self, height: String) -> Result<MinerIDs> { // why is height a string?
        self.send(&RpcRequest::new1(
            "minerids",
            height
        ))
    }

    fn notaries(&self, height: String) -> Result<Notaries> { // why is height a string?
        self.send(&RpcRequest::new1(
            "notaries",
            height
        ))
    }

    fn get_info(&self) -> Result<Info> {
        self.send(&RpcRequest::new0(
            "getinfo"
        ))
    }

    fn get_block_subsidy(&self, height: Option<u64>) -> Result<BlockSubsidy> {
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

    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<BlockTemplate> {
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

    fn get_local_sol_ps(&self) -> Result<f64> {
        self.send(&RpcRequest::new0(
            "getlocalsolps"
        ))
    }

    fn get_mining_info(&self) -> Result<MiningInfo> {
        self.send(&RpcRequest::new0(
            "getmininginfo"
        ))
    }

    fn get_network_hash_ps(&self) -> Result<u64> {
        self.send(&RpcRequest::new0(
            "getnetworkhashps"
        ))
    }

    fn get_network_sol_ps(&self) -> Result<u64> {
        self.send(&RpcRequest::new0(
            "getnetworksolps"
        ))
    }

    // todo untested
    fn prioritise_transaction(&self, txid: TransactionId, prio_delta: f64, fee_delta: u32) -> Result<bool> {
        self.send(&RpcRequest::new3(
            "prioritisetransaction",
            txid,
            prio_delta,
            fee_delta
        ))
    }

    // todo untested
    fn submit_block(&self, hexdata: String, jsonparametersobject: Option<ParametersObject>) -> Result<SubmitBlockResult> {
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

//    fn addnode(&self, node: String, action: AddNodeCommand) -> Result<()> {
//        self.send(&RpcRequest::new2(
//            "addnode",
//            node,
//            action
//        ))
//    }

//    fn clear_banned(&self) -> Result<()> {
//        self.send(&RpcRequest::new0(
//            "clearbanned",
//        ))
//    }

//    fn disconnect_node(&self, node: String) -> Result<()> {
//        self.send(&RpcRequest::new1(
//            "addnode",
//            node
//        ))
//    }

    fn get_added_node_info(&self, dns: bool, node: Option<&str>) -> Result<Vec<AddedNodeInfo>> {
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

    fn get_connection_count(&self) -> Result<u32> {
        self.send(&RpcRequest::new0(
            "getconnectioncount"
        ))
    }

    fn get_deprecation_info(&self) -> Result<DeprecationInfo> {
        self.send(&RpcRequest::new0(
            "getdeprecationinfo"
        ))
    }

    fn get_net_totals(&self) -> Result<NetTotals> {
        self.send(&RpcRequest::new0(
            "getnettotals"
        ))
    }

    fn get_network_info(&self) -> Result<NetworkInfo> {
        self.send(&RpcRequest::new0(
            "getnetworkinfo"
        ))
    }

    fn get_peer_info(&self) -> Result<Vec<Peer>> {
        self.send(&RpcRequest::new0(
            "getpeerinfo"
        ))
    }

    fn list_banned(&self) -> Result<Vec<Option<BannedNode>>> {
        self.send(&RpcRequest::new0(
            "listbanned"
        ))
    }

//    fn ping(&self) -> Result<()> {
//        self.send(&RpcRequest::new0(
//            "ping"
//        ))
//    }

    fn set_ban(
        &self,
        ip: String,
        command: String,
        bantime: Option<u32>,
        absolute: Option<bool>
    ) -> Result<()> {
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

    fn create_raw_transaction(&self, inputs: CreateRawTransactionInputs, outputs: CreateRawTransactionOutputs) -> Result<SerializedRawTransaction> {
        self.send(&RpcRequest::new2(
            "createrawtransaction",
            inputs,
            outputs
        ))
    }

    fn decode_raw_transaction(&self, hexstring: &str) -> Result<RawTransaction> {
        self.send(&RpcRequest::new1(
            "decoderawtransaction",
            hexstring
        ))
    }

    fn decode_script(&self, hexstring: &str) -> Result<DecodedScript> {
        self.send(&RpcRequest::new1(
            "decodescript",
            hexstring
        ))
    }

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<SerializedRawTransaction> {
        self.send(&RpcRequest::new1(
            "getrawtransaction",
            txid
        ))
    }

    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<RawTransaction> {
        self.send(&RpcRequest::new2(
            "getrawtransaction",
            txid,
            1
        ))
    }

    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<SignedRawTransaction> {
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
    ) -> Result<SignedRawTransaction> {
        self.send(&RpcRequest::new4(
            "signrawtransaction",
            hexstring,
            txoutput_detail,
            private_keys,
            signature_hash_type
        ))
    }

    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<TransactionId> {
        self.send(&RpcRequest::new1(
            "sendrawtransaction",
            &signed_tx.hex
        ))
    }

    fn createmultisig(&self, nrequired: u8, keys: Vec<&str>) -> Result<Multisig> {
        self.send(&RpcRequest::new2(
            "createmultisig",
            nrequired,
            keys
        ))
    }

    fn backup_wallet(&self, file_name: &str) -> Result<String> {
        self.send(&RpcRequest::new1(
            "backupwallet",
            file_name
        ))
    }

    fn dump_privkey(&self, address: &str) -> Result<String> {
        self.send(&RpcRequest::new1(
            "dumpprivkey",
            address
        ))
    }

    fn dump_wallet(&self, filename: &str) -> Result<String> {
        self.send(&RpcRequest::new1(
            "dumpwallet",
            filename
        ))
    }

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<f64> {
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

    fn get_new_address(&self) -> Result<String> {
        self.send(&RpcRequest::new0(
            "getnewaddress",
        ))
    }

    fn get_raw_change_address(&self) -> Result<Address> {
        self.send(&RpcRequest::new0(
            "getrawchangeaddress",
        ))
    }

    fn get_received_by_address(&self, address: &Address) -> Result<f64> {
        self.send(&RpcRequest::new1(
            "getreceivedbyaddress",
            &address.addr
        ))
    }

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Transaction> {
        self.send(&RpcRequest::new1(
            "gettransaction",
            tx,
        ))
    }

    fn get_unconfirmed_balance(&self) -> Result<f64> {
        self.send(&RpcRequest::new0(
            "getunconfirmedbalance"
        ))
    }

    fn get_wallet_info(&self) -> Result<WalletInfo> {
        self.send(&RpcRequest::new0(
            "getwalletinfo"
        ))
    }

    fn import_address(&self, address: &Address, label: Option<String>, rescan: bool) -> Result<()> {
//        match label {
//            Some(label) => self.send(&RpcRequest::new3(
//                "importaddress",
//                &label,
//                rescan
//            )),
//            None => self.send(&RpcRequest::new3(
//                "importaddress",
//                "",
//                rescan
//            ))
//        }
        unimplemented!()
    }

    fn import_privkey(&self, key: &str, label: Option<&str>, rescan: bool) -> Result<Address> {
        match label {
            Some(label) => self.send(&RpcRequest::new3(
                "importaddress",
                key,
                &label,
                rescan
            )),
            None => self.send(&RpcRequest::new3(
                "importaddress",
                key,
                "",
                rescan
            ))
        }
    }


    fn import_wallet(&self, path: &str) -> Result<()> {
        unimplemented!()
    }

    fn list_address_groupings(&self) -> Result<AddressGrouping> {
//        self.send(&RpcRequest::new0(
//            "listaddressgroupings"
//        ))
        unimplemented!()
    }

    fn list_lock_unspent(&self) -> Result<Vec<LockedUnspent>> {
        self.send(&RpcRequest::new0(
            "listlockunspent"
        ))
    }

    fn list_received_by_address(&self, minconf: Option<u32>, include_empty: Option<bool>, include_watch_only: Option<bool>) -> Result<Vec<ReceivedByAddress>> {
        match (minconf, include_empty, include_watch_only) {
            (Some(min), None, None) => self.send(&RpcRequest::new1(
                "listreceivedbyaddress",
                min
            )),
            (None, Some(b), None) => self.send(&RpcRequest::new2(
                "listreceivedbyaddress",
                1,
                b
            )),
            (Some(min), Some(b), None) => self.send(&RpcRequest::new2(
                "listreceivedbyaddress",
                min,
                b
            )),
            (None, None, Some(wo)) => self.send(&RpcRequest::new3(
                "listreceivedbyaddress",
                1,
                false,
                wo
            )),
            (None, Some(b), Some(wo)) => self.send(&RpcRequest::new3(
                "listreceivedbyaddress",
                1,
                b,
                wo
            )),
            (Some(min), None, Some(wo)) => self.send(&RpcRequest::new3(
                "listreceivedbyaddress",
                min,
                false,
                wo
            )),
            (Some(min), Some(b), Some(wo)) => self.send(&RpcRequest::new3(
                "listreceivedbyaddress",
                min,
                b,
                wo
            )),
            _ => self.send(&RpcRequest::new0(
                "listreceivedbyaddress",
            ))
        }
    }

    fn list_since_block(&self, blockhash: Option<&str>, confs: Option<u64>, include_watch_only: Option<bool>) -> Result<TxListSinceBlock> {
        match (blockhash, confs, include_watch_only) {
            (Some(hash), None, None) => self.send(&RpcRequest::new1(
                "listsinceblock",
                hash
            )),
            (Some(hash), Some(confs), None) => self.send(&RpcRequest::new2(
                "listsinceblock",
                hash,
                confs
            )),
            (Some(hash), None, Some(watchonly)) => self.send(&RpcRequest::new3(
                "listsinceblock",
                hash,
                1,
                watchonly
            )),
            (Some(hash), Some(confs), Some(watchonly)) => self.send(&RpcRequest::new3(
                "listsinceblock",
                hash,
                confs,
                watchonly
            )),
            (None, Some(confs), None) => self.send(&RpcRequest::new2(
                "listsinceblock",
                "",
                confs
            )),
            (None, Some(confs), Some(watchonly)) => self.send(&RpcRequest::new3(
                "listsinceblock",
                "",
                confs,
                watchonly
            )),
            (None, None, Some(watchonly)) => self.send(&RpcRequest::new3(
                "listsinceblock",
                "",
                1,
                watchonly
            )),
            _ => self.send(&RpcRequest::new0(
                "listsinceblock"
            )),
        }
    }

    fn list_transactions(&self, count: Option<u32>, from: Option<u32>, include_watch_only: Option<bool>) -> Result<ListTransactions> {
        match (count, from, include_watch_only) {
            (Some(count), None, None) => self.send(&RpcRequest::new2(
                "listtransactions",
                "",
                count
            )),
            (Some(count), Some(from), None) => self.send(&RpcRequest::new3(
                "listtransactions",
                "",
                count,
                from
            )),
            (Some(count), None, Some(watchonly)) => self.send(&RpcRequest::new4(
                "listtransactions",
                "",
                count,
                0,
                watchonly
            )),
            (Some(count), Some(from), Some(watchonly)) => self.send(&RpcRequest::new4(
                "listtransactions",
                "",
                count,
                from,
                watchonly
            )),
            (None, Some(from), None) => self.send(&RpcRequest::new3(
                "listtransactions",
                "",
                10,
                from
            )),
            (None, None, Some(watchonly)) => self.send(&RpcRequest::new4(
                "listtransactions",
                "",
                10,
                0,
                watchonly
            )),
            (None, Some(from), Some(watchonly)) => self.send(&RpcRequest::new4(
                "listtransactions",
                "",
                10,
                from,
                watchonly
            )),
            _ => self.send(&RpcRequest::new0(
                "listtransactions"
            )),
        }
    }

    fn list_unspent(&self, minconf: Option<u32>, maxconf: Option<u32>, addr_filter: Option<Vec<Address>>) -> Result<Vec<Unspent>> {
        match (minconf, maxconf, addr_filter) {
            (None, None, Some(filter)) => self.send(&RpcRequest::new3(
                "listunspent",
                1,
                9999999,
                filter
            )),
            (Some(min), None, Some(filter)) => self.send(&RpcRequest::new3(
                "listunspent",
                min,
                9999999,
                filter
            )),
            (None, Some(max), Some(filter)) => self.send(&RpcRequest::new3(
                "listunspent",
                1,
                max,
                filter
            )),
            (Some(min), Some(max), Some(filter)) => self.send(&RpcRequest::new3(
                "listunspent",
                min,
                max,
                filter
            )),
            (Some(min), None, None) => self.send(&RpcRequest::new2(
                "listunspent",
                min,
                9999999
            )),
            (None, Some(max), None) => self.send(&RpcRequest::new2(
                "listunspent",
                1,
                max
            )),
            (Some(min), Some(max), None) => self.send(&RpcRequest::new2(
                "listunspent",
                min,
                max
            )),
            _ => self.send(&RpcRequest::new0(
                "listunspent"
            ))
        }
    }

    fn lock_unspent(&self, unlock: bool, txns: Vec<LockedUnspent>) -> Result<bool> {
        self.send(&RpcRequest::new2(
            "lockunspent",
            unlock,
            txns
        ))
    }

    fn resend_wallet_transactions(&self) -> Result<ResentWalletTransactions> {
        self.send(&RpcRequest::new0(
            "resendwallettransactions"
        ))
    }

    fn send_many(&self, amounts: SendManyAmounts, minconf: Option<u32>, comment: Option<&str>, subtractfeefromaddresses: Option<Vec<Address>>) -> Result<TransactionId> {
        match (minconf, comment, subtractfeefromaddresses) {
            (Some(minconf), None, None) => self.send(&RpcRequest::new3(
                "sendmany",
                "",
                amounts,
                minconf
            )),
            (Some(minconf), Some(comment), None) => self.send(&RpcRequest::new4(
                "sendmany",
                "",
                amounts,
                minconf,
                comment
            )),
            (Some(minconf), None, Some(fees)) => self.send(&RpcRequest::new5(
                "sendmany",
                "",
                amounts,
                minconf,
                "",
                fees
            )),
            (Some(minconf), Some(comment), Some(fees)) => self.send(&RpcRequest::new5(
                "sendmany",
                "",
                amounts,
                minconf,
                comment,
                fees
            )),
            (None, Some(comment), None) => self.send(&RpcRequest::new4(
                "sendmany",
                "",
                amounts,
                1,
                comment
            )),
            (None, None, Some(fees)) => self.send(&RpcRequest::new5(
                "sendmany",
                "",
                amounts,
                1,
                "",
                fees
            )),
            (None, Some(comment), Some(fees)) => self.send(&RpcRequest::new5(
                "sendmany",
                "",
                amounts,
                1,
                comment,
                fees
            )),
            _ => self.send(&RpcRequest::new2(
                "sendmany",
                "",
                amounts
            )),
        }
    }

    fn send_to_address(&self, address: Address, amount: f64, comment: Option<&str>, comment_to: Option<&str>, subtractfee: Option<bool>) -> Result<TransactionId> {
        match address.addr_type {
            AddrType::Transparent => {
                match (comment, comment_to, subtractfee) {
                    (Some(comment), None, None) => self.send(&RpcRequest::new3(
                        "sendtoaddress",
                        address,
                        amount,
                        comment
                    )),
                    (Some(comment), Some(comment_to), None) => self.send(&RpcRequest::new4(
                        "sendtoaddress",
                        address,
                        amount,
                        comment,
                        comment_to
                    )),
                    (Some(comment), None, Some(subtractfee)) => self.send(&RpcRequest::new5(
                        "sendtoaddress",
                        address,
                        amount,
                        comment,
                        "",
                        subtractfee
                    )),
                    (Some(comment), Some(comment_to), Some(subtractfee)) => self.send(&RpcRequest::new5(
                        "sendtoaddress",
                        address,
                        amount,
                        comment,
                        comment_to,
                        subtractfee
                    )),
                    (None, Some(comment_to), None) => self.send(&RpcRequest::new4(
                        "sendtoaddress",
                        address,
                        amount,
                        "",
                        comment_to
                    )),
                    (None, None, Some(subtractfee)) => self.send(&RpcRequest::new5(
                        "sendtoaddress",
                        address,
                        amount,
                        "",
                        "",
                        subtractfee
                    )),
                    (None, Some(comment_to), Some(subtractfee)) => self.send(&RpcRequest::new5(
                        "sendtoaddress",
                        address,
                        amount,
                        "",
                        comment_to,
                        subtractfee
                    )),
                    _ => self.send(&RpcRequest::new2(
                        "sendtoaddress",
                        address,
                        amount
                    ))
                }
            },
            _ => Err(ApiError::Other(String::from("address is not transparent")))
        }
    }

    fn set_pubkey(&self, pubkey: &str) -> Result<SetPubkey> {
//        self.send(&RpcRequest::new1(
//            "setpubkey",
//            pubkey
//        ))
        unimplemented!()
    }

    fn set_txfee(&self, fee: f64) -> Result<bool> {
        self.send(&RpcRequest::new1(
            "settxfee",
            fee
        ))
    }

    fn sign_message(&self, address: Address, message: &str) -> Result<String> {
        self.send(&RpcRequest::new2(
            "signmessage",
            address,
            message
        ))
    }

    fn z_exportkey(&self, a: &Address) -> Result<PrivateKey> {
        match a.addr_type {
            AddrType::Shielded => self.send(&RpcRequest::new1(
                "z_exportkey",
                &a.addr
            )),
            AddrType::Transparent => Err(ApiError::Other("transparent key not supported in z_exportkey".to_string()))
        }
    }

    fn z_exportviewingkey(&self, a: &Address) -> Result<ViewingKey> {
//        match a.addr_type {
//            AddrType::Shielded => self.send(&RpcRequest::new1(
//                "z_exportviewingkey",
//                &a.addr
//            )),
//            AddrType::Transparent => Err(ApiError::Other("transparent key not supported in z_exportviewingkey".to_string()))
//        }
        unimplemented!()
    }

    fn z_exportwallet(&self, s: &str) -> Result<String> {
        self.send(&RpcRequest::new1(
            "z_exportwallet",
            s
        ))
    }

    fn z_getbalance(&self, addr: &Address, minconf: Option<u32>) -> Result<f64> {
        match minconf {
            Some(conf) => self.send(&RpcRequest::new2(
                "z_getbalance",
                &addr.addr,
                conf
            )),
            None => self.send(&RpcRequest::new1(
                "z_getbalance",
                &addr.addr
            ))
        }
    }

    fn z_getnewaddress(&self) -> Result<Address> {
        self.send(&RpcRequest::new0(
            "z_getnewaddress"
        ))
    }

    fn z_getoperationresult(&self, v: Vec<&str>) -> Result<Operations> {
        self.send(&RpcRequest::new1(
            "z_getoperationresult",
            v
        ))
    }

    fn z_getoperationstatus(&self, v: Vec<&str>) -> Result<Operations> {
        self.send(&RpcRequest::new1(
            "z_getoperationstatus",
            v
        ))
    }

    fn z_gettotalbalance(&self) -> Result<TotalBalance> {
        self.send(&RpcRequest::new0(
            "z_gettotalbalance"
        ))
    }

    fn z_importkey(&self) -> Result<()> {
        unimplemented!()
    }

    fn z_importviewingkey(&self) -> Result<()> {
        unimplemented!()
    }

    fn z_importwallet(&self) -> Result<()> {
        unimplemented!()
    }

    fn z_listaddresses(&self, include_watch_only: Option<bool>) -> Result<Vec<Address>> {
        match include_watch_only {
            Some(b) => self.send(&RpcRequest::new1(
                "z_listaddresses",
                b
            )),
            None => self.send(&RpcRequest::new0(
                "z_listaddresses"
            ))
        }
    }

    fn z_listoperationids(&self) -> Result<Vec<String>> {
        self.send(&RpcRequest::new0(
            "z_listoperationids"
        ))
    }

    fn z_listreceivedbyaddress(&self, a: &Address, minconf: Option<u32>) -> Result<ReceivedByAddress> {
        match a.addr_type {
            AddrType::Shielded => self.send(&RpcRequest::new2(
                "z_listreceivedbyaddress",
                &a.addr,
                match minconf {
                    Some(conf) => conf,
                    None => 1
                }
            )),
            _ => Err(ApiError::Other(format!("Not a Shielded address: {}", a.addr))) // ApiError::Parameter
        }
    }

    fn z_mergetoaddress(
        &self,
        from_addresses: &FromAddresses,
        to_address: &Address,
        fee: Option<f64>,
        transparent_limit: Option<u32>,
        shielded_limit: Option<u32>,
        maximum_utxo_size: Option<u64>,
        memo: Option<String>
    ) -> Result<MergeResult> {
        match (maximum_utxo_size, memo) {
            (Some(size), None) => self.send(&RpcRequest::new6(
                "z_mergetoaddress",
                &from_addresses.0,
                &to_address.addr,
                match fee {
                    Some(fee) => fee,
                    None => 0.0001,
                },
                match transparent_limit {
                    Some(limit) => limit,
                    None => 50
                },
                match shielded_limit {
                    Some(limit) => limit,
                    None => 90
                },
                size
            )),
            (Some(size), Some(ref memo)) => self.send(&RpcRequest::new7(
                "z_mergetoaddress",
                &from_addresses.0,
                &to_address.addr,
                match fee {
                    Some(fee) => fee,
                    None => 0.0001,
                },
                match transparent_limit {
                    Some(limit) => limit,
                    None => 50
                },
                match shielded_limit {
                    Some(limit) => limit,
                    None => 90
                },
                size,
                memo
            )),
            (None, Some(_)) =>
                Err(ApiError::Other("cannot specify memo without specifying maximum_utxo_size".to_string())),
            _ => self.send(&RpcRequest::new5(
                "z_mergetoaddress",
                &from_addresses.0,
                &to_address.addr,
                match fee {
                    Some(fee) => fee,
                    None => 0.0001,
                },
                match transparent_limit {
                    Some(limit) => limit,
                    None => 50
                },
                match shielded_limit {
                    Some(limit) => limit,
                    None => 90
                }
            ))
        }
    }

    fn z_sendmany(
        &self,
        from_address: &Address,
        amounts: &Amounts,
        minconf: Option<u32>,
        fee: Option<f64>,
    ) -> Result<String> {
        self.send(&RpcRequest::new4(
            "z_sendmany",
            &from_address.addr,
            &amounts.0,
            match minconf {
                Some(conf) => conf,
                None => 1,
            },
            match fee {
                Some(fee) => fee,
                None => 0.0001
            }
        ))
    }

    fn z_shieldcoinbase(&self, from_address: &Address, to_address: &Address, fee: Option<f64>, limit: Option<u32>) -> Result<ShieldResult> {
        self.send(&RpcRequest::new4(
            "z_shieldcoinbase",
            &from_address,
            &to_address,
            match fee {
                Some(fee) => fee,
                None => 0.0001,
            },
            match limit {
                Some(limit) => limit,
                None => 50
            }
        ))
    }
}