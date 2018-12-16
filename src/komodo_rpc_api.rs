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

    fn decode_raw_transaction(&self, hex_string: &str) -> Result<RawTransaction, ApiError>;
    fn get_raw_transaction(&self, txid: arguments::TransactionId) -> Result<String, ApiError>; // todo returns serialized transaction
    fn get_raw_transaction_verbose(&self, txid: arguments::TransactionId) -> Result<RawTransaction, ApiError>;


    fn backup_wallet(&self, file_name: &str) -> Result<String, ApiError>;


    fn dump_privkey(&self, address: &str) -> Result<String, ApiError>;
    fn dump_wallet(&self, filename: &str) -> Result<String, ApiError>;


    fn get_balance(&self, minconf: Option<u32>, include_watchonly: Option<bool>) -> Result<f64, ApiError>;
    fn get_new_address(&self) -> Result<String, ApiError>;
    fn get_raw_change_address(&self) -> Result<String, ApiError>;

    fn get_transaction(&self, tx: &TransactionId) -> Result<Transaction, ApiError>;

    fn get_wallet_info(&self) -> Result<WalletInfo, ApiError>;
}