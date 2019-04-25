use ::{TransactionId, ApiError};
use BlockHash;
use types::*;
use types::arguments::address::{Address, FromAddresses};
use arguments::address::Amounts;
use types::arguments::SendManyAmounts;

type Result<T> = std::result::Result<T, ApiError>;


pub trait KomodoRpcApi {
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<AddressBalance>;
    fn get_address_deltas(&self, addresses: &arguments::AddressList)  -> Result<AddressDeltas>;
    fn get_address_mempool(&self, addresses: &arguments::AddressList) -> Result<AddressMempool>;
    fn get_address_tx_ids(&self, addresses: &arguments::AddressList)  -> Result<AddressTxIDs>;
    fn get_address_utxos(&self, addresses: &arguments::AddressList)   -> Result<AddressUtxos>;

    // getting a snapshot takes an optional parameter. need to create 2 API calls:
    fn get_snapshot_max(&self, n: u32) -> Result<Snapshot>;
    fn get_snapshot(&self) -> Result<Snapshot>;

    fn coinsupply(&self, n: u32) -> Result<Coinsupply>;
    fn get_best_block_hash(&self) -> Result<BlockHash>;
    fn get_block(&self, hashorheight: String) -> Result<Block>;

    fn get_blockchain_info(&self) -> Result<BlockchainInfo>;
    fn get_block_count(&self) -> Result<Blockcount>;
    fn get_block_hash(&self, n: u32) -> Result<BlockHash>;
    fn get_block_header(&self, hash: String) -> Result<BlockHeader>;
    fn get_chaintips(&self) -> Result<ChainTips>;
    fn get_difficulty(&self) -> Result<f64>;
    fn get_mempool_info(&self) -> Result<MempoolInfo>;
    fn get_raw_mempool(&self) -> Result<RawMempool>;
    fn get_raw_mempool_verbose(&self) -> Result<RawMempoolVerbose>;
    fn get_tx_out(&self, txid: String, index: u8) -> Result<TxOut>;

    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo>;
    fn minerids(&self, height: String) -> Result<MinerIDs>;

    fn notaries(&self, height: String) -> Result<Notaries>;
    fn get_info(&self) -> Result<Info>;

    // mining
    fn get_block_subsidy(&self, height: Option<u64>) -> Result<BlockSubsidy >;
    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<BlockTemplate >;
    fn get_local_sol_ps(&self) -> Result<f64 >;
    fn get_mining_info(&self) -> Result<MiningInfo >;
    fn get_network_hash_ps(&self) -> Result<u64 >;
    fn get_network_sol_ps(&self) -> Result<u64 >;
    fn prioritise_transaction(&self, txid: TransactionId, prio_delta: f64, fee_delta: u32) -> Result<bool>;
    fn submit_block(&self, hexdata: String, jsonparametersobject: Option<ParametersObject>) -> Result<SubmitBlockResult>;

    // network
//    fn addnode(&self, node: String, action: AddNodeCommand) -> Result<()>; // node is an ip address | komodod returns an empty response
//    fn clear_banned(&self) -> Result<()>; // komodod returns an empty response
//    fn disconnect_node(&self, node: String) -> Result<()>; // node is an ip address | komodod returns an empty response
    fn get_added_node_info(&self, dns: bool, node: Option<&str>) -> Result<Vec<AddedNodeInfo>>;
    fn get_connection_count(&self) -> Result<u32>;
    fn get_deprecation_info(&self) -> Result<DeprecationInfo>;
    fn get_net_totals(&self) -> Result<NetTotals>;
    fn get_network_info(&self) -> Result<NetworkInfo>;
    fn get_peer_info(&self) -> Result<Vec<Peer>>;
    fn list_banned(&self) -> Result<Vec<Option<BannedNode>>>;
//    fn ping(&self) -> Result<()>;
    fn set_ban(&self, ip: String, command: String, bantime: Option<u32>, absolute: Option<bool>) -> Result<()>; //"ip(/netmask)" "add|remove" (bantime) (absolute)

    fn create_raw_transaction(&self, inputs: arguments::CreateRawTransactionInputs, outputs: arguments::CreateRawTransactionOutputs) -> Result<SerializedRawTransaction>;
    fn decode_raw_transaction(&self, hexstring: &str) -> Result<RawTransaction>;
    fn decode_script(&self, hexstring: &str) -> Result<DecodedScript>;

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<SerializedRawTransaction>; // todo returns serialized transaction
    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<RawTransaction>;
    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<SignedRawTransaction>;
    fn sign_raw_transaction_with_key(
        &self,
        hexstring: &SerializedRawTransaction,
        txoutput_detail: Option<Vec<TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<SignedRawTransaction>;
    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<TransactionId>;

    fn createmultisig(&self, nrequired: u8, keys: Vec<&str>) -> Result<Multisig>;

    fn backup_wallet(&self, file_name: &str) -> Result<String>;

    fn dump_privkey(&self, address: &str) -> Result<String>;
    fn dump_wallet(&self, filename: &str) -> Result<String>;

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<f64>;
    fn get_new_address(&self) -> Result<String>;
    fn get_raw_change_address(&self) -> Result<Address>;
    fn get_received_by_address(&self, address: &Address) -> Result<f64>;
    fn get_transaction(&self, tx: &TransactionId) -> Result<Transaction>;
    fn get_unconfirmed_balance(&self) -> Result<f64>;
    fn get_wallet_info(&self) -> Result<WalletInfo>;
    fn import_address(&self, address: &Address, label: Option<String>, rescan: bool) -> Result<()>;
    fn import_privkey(&self, key: &str, label: Option<&str>, rescan: bool) -> Result<Address>;
    fn import_wallet(&self, path: &str) -> Result<()>;
    fn list_address_groupings(&self) -> Result<AddressGrouping>;
    fn list_lock_unspent(&self) -> Result<Vec<LockedUnspent>>;
    fn list_received_by_address(&self, minconf: Option<u32>, include_empty: Option<bool>, include_watch_only: Option<bool>) -> Result<Vec<ReceivedByAddress>>;
    fn list_since_block(&self, blockhash: Option<&str>, confs: Option<u64>, include_watch_only: Option<bool>) -> Result<TxListSinceBlock>;
    fn list_transactions(&self, count: Option<u32>, from: Option<u32>, include_watch_only: Option<bool>) -> Result<ListTransactions>;
    fn list_unspent(&self, minconf: Option<u32>, maxconf: Option<u32>, addr_filter: Option<Vec<Address>>) -> Result<Vec<Unspent>>;
    fn lock_unspent(&self, unlock: bool, txns: Vec<LockedUnspent>) -> Result<bool>;
    fn resend_wallet_transactions(&self) -> Result<ResentWalletTransactions>;
    fn send_many(&self, amounts: SendManyAmounts, minconf: Option<u32>, comment: Option<&str>, subtractfeefromaddresses: Option<Vec<Address>>) -> Result<TransactionId>;
    fn send_to_address(&self, address: Address, amount: f64, comment: Option<&str>, comment_to: Option<&str>, subtractfee: Option<bool>) -> Result<TransactionId>;
    fn set_pubkey(&self, pubkey: &str) -> Result<SetPubkey>;
    fn set_txfee(&self, fee: f64) -> Result<bool>;
    fn sign_message(&self, address: Address, message: &str) -> Result<String>;

    fn z_exportkey(&self, a: &Address) -> Result<PrivateKey>;
    fn z_exportviewingkey(&self, a: &Address) -> Result<ViewingKey>; //todo unsupported https://github.com/zcash/zcash/issues/3060
    fn z_exportwallet(&self, filename: &str) -> Result<String>;
    fn z_getbalance(&self, a: &Address, minconf: Option<u32>) -> Result<f64>;
    fn z_getnewaddress(&self) -> Result<Address>; // type parameter unsupported
    fn z_getoperationresult(&self, v: Vec<&str>) -> Result<Operations>;
    fn z_getoperationstatus(&self, v: Vec<&str>) -> Result<Operations>;
    fn z_gettotalbalance(&self) -> Result<TotalBalance>;
    fn z_importkey(&self) -> Result<()>;
    fn z_importviewingkey(&self) -> Result<()>;
    fn z_importwallet(&self) -> Result<()>;
    fn z_listaddresses(&self, include_watch_only: Option<bool>) -> Result<Vec<Address>>;
    fn z_listoperationids(&self) -> Result<Vec<String>>;
    fn z_listreceivedbyaddress(&self, a: &Address, minconf: Option<u32>) -> Result<ReceivedByAddress>; // todo check beforehand if addy is shielded!
    fn z_mergetoaddress(
        &self,
        from_addresses: &FromAddresses,
        to_address: &Address,
        fee: Option<f64>,
        transparent_limit: Option<u32>,
        shielded_limit: Option<u32>,
        maximum_utxo_size: Option<u64>,
        memo: Option<String>
    ) -> Result<MergeResult>;
    fn z_sendmany(&self, from_address: &Address, amounts: &Amounts, minconf: Option<u32>, fee: Option<f64>,) -> Result<String>;
    fn z_shieldcoinbase(&self, from_address: &Address, to_address: &Address, fee: Option<f64>, limit: Option<u32>) -> Result<ShieldResult>;
}