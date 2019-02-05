use TransactionId;
use BlockHash;
use error::ApiError;
use rpc::*;


pub trait KomodoRpcApi {
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<Option<AddressBalance>, ApiError>;
    fn get_address_deltas(&self, addresses: &arguments::AddressList)  -> Result<Option<AddressDeltas>,  ApiError>;
    fn get_address_mempool(&self, addresses: &arguments::AddressList) -> Result<Option<AddressMempool>, ApiError>;
    fn get_address_tx_ids(&self, addresses: &arguments::AddressList)  -> Result<Option<AddressTxIDs>,   ApiError>;
    fn get_address_utxos(&self, addresses: &arguments::AddressList)   -> Result<Option<AddressUtxos>,   ApiError>;

    // getting a snapshot takes an optional parameter. need to create 2 API calls:
    fn get_snapshot_max(&self, n: u32) -> Result<Option<Snapshot>, ApiError>;
    fn get_snapshot(&self) -> Result<Option<Snapshot>, ApiError>;

    fn coinsupply(&self, n: u32) -> Result<Option<Coinsupply>, ApiError>;
    fn get_best_block_hash(&self) -> Result<Option<BlockHash>, ApiError>;
    fn get_block(&self, hashorheight: String) -> Result<Option<Block>, ApiError>;

    fn get_blockchain_info(&self) -> Result<Option<BlockchainInfo>, ApiError>;
    fn get_block_count(&self) -> Result<Option<Blockcount>, ApiError>;
    fn get_block_hash(&self, n: u32) -> Result<Option<BlockHash>, ApiError>;
    fn get_block_header(&self, hash: String) -> Result<Option<BlockHeader>, ApiError>;
    fn get_chaintips(&self) -> Result<Option<ChainTips>, ApiError>;
    fn get_difficulty(&self) -> Result<Option<f64>, ApiError>;
    fn get_mempool_info(&self) -> Result<Option<MempoolInfo>, ApiError>;
    fn get_raw_mempool(&self) -> Result<Option<RawMempool>, ApiError>;
    fn get_raw_mempool_verbose(&self) -> Result<Option<RawMempoolVerbose>, ApiError>;
    fn get_tx_out(&self, txid: String, index: u8) -> Result<Option<TxOut>, ApiError>;

    fn get_tx_out_set_info(&self) -> Result<Option<TxOutSetInfo>, ApiError>;
    fn minerids(&self, height: String) -> Result<Option<MinerIDs>, ApiError>;

    fn notaries(&self, height: String) -> Result<Option<Notaries>, ApiError>;
    fn get_info(&self) -> Result<Option<Info>, ApiError>;

    // mining
    fn get_block_subsidy(&self, height: Option<u64>) -> Result<Option<BlockSubsidy >, ApiError>;
    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<Option<BlockTemplate >, ApiError>;
    fn get_local_sol_ps(&self) -> Result<Option<f64 >, ApiError>;
    fn get_mining_info(&self) -> Result<Option<MiningInfo >, ApiError>;
    fn get_network_hash_ps(&self) -> Result<Option<u64 >, ApiError>;
    fn get_network_sol_ps(&self) -> Result<Option<u64 >, ApiError>;
    fn prioritise_transaction(&self, txid: TransactionId, prio_delta: f64, fee_delta: u32) -> Result<Option<bool>, ApiError>;
    fn submit_block(&self, hexdata: String, jsonparametersobject: Option<ParametersObject>) -> Result<Option<SubmitBlockResult>, ApiError>;

    // network
    fn addnode(&self, node: String, action: AddNodeCommand) -> Result<Option<()>, ApiError>; // node is an ip address | komodod returns an empty response
    fn clear_banned(&self) -> Result<Option<()>, ApiError>; // komodod returns an empty response
    fn disconnect_node(&self, node: String) -> Result<Option<()>, ApiError>; // node is an ip address | komodod returns an empty response
    fn get_added_node_info(&self, dns: bool, node: Option<&str>) -> Result<Option<Vec<AddedNodeInfo>>, ApiError>;
    fn get_connection_count(&self) -> Result<Option<u32>, ApiError>;
    fn get_deprecation_info(&self) -> Result<Option<DeprecationInfo>, ApiError>;
    fn get_net_totals(&self) -> Result<Option<NetTotals>, ApiError>;
    fn get_network_info(&self) -> Result<Option<NetworkInfo>, ApiError>;
    fn get_peer_info(&self) -> Result<Option<Vec<Peer>>, ApiError>;
    fn list_banned(&self) -> Result<Option<Vec<Option<BannedNode>>>, ApiError>;
    fn ping(&self) -> Result<Option<()>, ApiError>;
    fn set_ban(&self, ip: String, command: String, bantime: Option<u32>, absolute: Option<bool>) -> Result<Option<()>, ApiError>; //"ip(/netmask)" "add|remove" (bantime) (absolute)

    fn create_raw_transaction(&self, inputs: arguments::CreateRawTransactionInputs, outputs: arguments::CreateRawTransactionOutputs) -> Result<Option<SerializedRawTransaction>, ApiError>;
    fn decode_raw_transaction(&self, hexstring: &str) -> Result<Option<RawTransaction>, ApiError>;
    fn decode_script(&self, hexstring: &str) -> Result<Option<DecodedScript>, ApiError>;

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<Option<SerializedRawTransaction>, ApiError>; // todo returns serialized transaction
    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<Option<RawTransaction>, ApiError>;
    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<Option<SignedRawTransaction>, ApiError>;
    fn sign_raw_transaction_with_key(
        &self,
        hexstring: &SerializedRawTransaction,
        txoutput_detail: Option<Vec<TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<Option<SignedRawTransaction>, ApiError>;
    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<Option<TransactionId>, ApiError>;

    fn backup_wallet(&self, file_name: &str) -> Result<Option<String>, ApiError>;

    fn dump_privkey(&self, address: &str) -> Result<Option<String>, ApiError>;
    fn dump_wallet(&self, filename: &str) -> Result<Option<String>, ApiError>;

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<Option<f64>, ApiError>;
    fn get_new_address(&self) -> Result<Option<String>, ApiError>;
    fn get_raw_change_address(&self) -> Result<Option<String>, ApiError>;

    fn get_transaction(&self, tx: &TransactionId) -> Result<Option<Transaction>, ApiError>;

    fn get_wallet_info(&self) -> Result<Option<WalletInfo>, ApiError>;
}