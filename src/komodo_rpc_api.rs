use ClientError;
use RpcError;
use TransactionId;
use BlockHash;
use error::ApiError;
use rpc::*;


pub trait KomodoRpcApi {
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<AddressBalance, ApiError>;
    fn get_address_deltas(&self, addresses: &arguments::AddressList)  -> Result<AddressDeltas,  ApiError>;
    fn get_address_mempool(&self, addresses: &arguments::AddressList) -> Result<AddressMempool, ApiError>;
    fn get_address_tx_ids(&self, addresses: &arguments::AddressList)  -> Result<AddressTxIDs,   ApiError>;
    fn get_address_utxos(&self, addresses: &arguments::AddressList)   -> Result<AddressUtxos,   ApiError>;

    // getting a snapshot takes an optional parameter. need to create 2 API calls:
    fn get_snapshot_max(&self, n: u32) -> Result<Snapshot, ApiError>;
    fn get_snapshot(&self) -> Result<Snapshot, ApiError>;

    fn coinsupply(&self, n: u32) -> Result<Coinsupply, ApiError>;
    fn get_best_block_hash(&self) -> Result<BlockHash, ApiError>;
    fn get_block(&self, hashorheight: String) -> Result<Block, ApiError>;

    fn get_blockchain_info(&self) -> Result<BlockchainInfo, ApiError>;
    fn get_block_count(&self) -> Result<Blockcount, ApiError>;
    fn get_block_hash(&self, n: u32) -> Result<BlockHash, ApiError>;
    fn get_block_header(&self, hash: String) -> Result<BlockHeader, ApiError>;
    fn get_chaintips(&self) -> Result<ChainTips, ApiError>;
    fn get_difficulty(&self) -> Result<f64, ApiError>;
    fn get_mempool_info(&self) -> Result<MempoolInfo, ApiError>;
    fn get_raw_mempool(&self) -> Result<RawMempool, ApiError>;
    fn get_raw_mempool_verbose(&self) -> Result<RawMempoolVerbose, ApiError>;
    fn get_tx_out(&self, txid: String, index: u8) -> Result<TxOut, ApiError>;

    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo, ApiError>;
    fn minerids(&self, height: String) -> Result<MinerIDs, ApiError>;

    fn notaries(&self, height: String) -> Result<Notaries, ApiError>;
    fn get_info(&self) -> Result<Info, ApiError>;

    // mining
    fn get_block_subsidy(&self, height: Option<u64>) -> Result<BlockSubsidy , ApiError>;
    fn get_block_template(&self, jsonrequestobject: Option<&RequestObject>) -> Result<BlockTemplate , ApiError>;
    fn get_local_sol_ps(&self) -> Result<f64 , ApiError>;
    fn get_mining_info(&self) -> Result<MiningInfo , ApiError>;
    fn get_network_hash_ps(&self) -> Result<u64 , ApiError>;
    fn get_network_sol_ps(&self) -> Result<u64 , ApiError>;
    fn prioritise_transaction(&self, txid: TransactionId, prio_delta: f64, fee_delta: u32) -> Result<bool, ApiError>;
    fn submit_block(&self, hexdata: String, jsonparametersobject: Option<ParametersObject>) -> Result<SubmitBlockResult, ApiError>;

    // network
    fn addnode(&self, node: String, action: AddNodeCommand) -> Result<(), ApiError>; // node is an ip address | komodod returns an empty response
    fn clear_banned(&self) -> Result<(), ApiError>; // komodod returns an empty response
    fn disconnect_node(&self, node: String) -> Result<(), ApiError>; // node is an ip address | komodod returns an empty response
    fn get_added_node_info(&self, dns: bool, node: Option<&str>) -> Result<Vec<AddedNodeInfo>, ApiError>;
//    fn get_connection_count(&self)
//    fn get_deprecation_info(&self)
//    fn get_net_totals(&self)
//    fn get_network_info(&self)
//    fn get_peer_info(&self)
//    fn list_banned(&self)
//    fn ping(&self)
//    fn set_ban(&self) "ip(/netmask)" "add|remove" (bantime) (absolute)

    fn create_raw_transaction(&self, inputs: arguments::CreateRawTransactionInputs, outputs: arguments::CreateRawTransactionOutputs) -> Result<SerializedRawTransaction, ApiError>;
    fn decode_raw_transaction(&self, hexstring: &str) -> Result<RawTransaction, ApiError>;
    fn decode_script(&self, hexstring: &str) -> Result<DecodedScript, ApiError>;

    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<SerializedRawTransaction, ApiError>; // todo returns serialized transaction
    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<RawTransaction, ApiError>;
    fn sign_raw_transaction_with_wallet(&self, hexstring: SerializedRawTransaction) -> Result<SignedRawTransaction, ApiError>;
    fn sign_raw_transaction_with_key(
        &self,
        hexstring: &SerializedRawTransaction,
        txoutput_detail: Option<Vec<&TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<SignedRawTransaction, ApiError>;
    fn send_raw_transaction(&self, signed_tx: &SignedRawTransaction) -> Result<TransactionId, ApiError>;

    fn backup_wallet(&self, file_name: &str) -> Result<String, ApiError>;

    fn dump_privkey(&self, address: &str) -> Result<String, ApiError>;
    fn dump_wallet(&self, filename: &str) -> Result<String, ApiError>;

    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<f64, ApiError>;
    fn get_new_address(&self) -> Result<String, ApiError>;
    fn get_raw_change_address(&self) -> Result<String, ApiError>;

    fn get_transaction(&self, tx: &TransactionId) -> Result<Transaction, ApiError>;

    fn get_wallet_info(&self) -> Result<WalletInfo, ApiError>;
}