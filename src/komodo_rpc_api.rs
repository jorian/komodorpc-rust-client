use ClientError;
use RpcError;
use TransactionId;
use BlockHash;
use serde::Serialize;
use std::fmt::Debug;

use rpc::*;
use std::path::Path;

pub trait KomodoRpcApi {
    /*
    Addressindex
*/
    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<Result<AddressBalance, RpcError>, ClientError>;
    fn get_address_deltas(&self, addresses: &arguments::AddressList)  -> Result<Result<AddressDeltas, RpcError>, ClientError>;
    fn get_address_mempool(&self, addresses: &arguments::AddressList) -> Result<Result<AddressMempool, RpcError>, ClientError>;
    fn get_address_tx_ids(&self, addresses: &arguments::AddressList)  -> Result<Result<AddressTxIDs, RpcError>, ClientError>;
    fn get_address_utxos(&self, addresses: &arguments::AddressList)   -> Result<Result<AddressUtxos, RpcError>, ClientError>;

    // getting a snapshot takes an optional parameter. need to create 2 API calls:
    fn get_snapshot_max(&self, n: u32) -> Result<Result<Snapshot, RpcError>, ClientError>;
    fn get_snapshot(&self) -> Result<Result<Snapshot, RpcError>, ClientError>;

    fn coinsupply(&self, n: u32) -> Result<Result<Coinsupply, RpcError>, ClientError>;
    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError>;
    fn get_block(&self, hashorheight: String) -> Result<Result<Block, RpcError>, ClientError>;

    fn get_blockchain_info(&self) -> Result<Result<BlockchainInfo, RpcError>, ClientError>;
    fn get_block_count(&self) -> Result<Result<Blockcount, RpcError>, ClientError>;
    fn get_wallet_info(&self) -> Result<Result<WalletInfo, RpcError>, ClientError>;
    fn get_block_hash(&self, n: u32) -> Result<Result<BlockHash, RpcError>, ClientError>;
    fn get_block_header(&self, hash: String) -> Result<Result<BlockHeader, RpcError>, ClientError>;
    fn get_chaintips(&self) -> Result<Result<ChainTips, RpcError>, ClientError>;
    fn get_difficulty(&self) -> Result<Result<f64, RpcError>, ClientError>;
    fn get_mempool_info(&self) -> Result<Result<MempoolInfo, RpcError>, ClientError>;
    fn get_raw_mempool(&self) -> Result<Result<RawMempool, RpcError>, ClientError>;
    fn get_raw_mempool_verbose(&self) -> Result<Result<RawMempoolVerbose, RpcError>, ClientError>;

    fn get_tx_out(&self, txid: String, index: u8) -> Result<Result<TxOut, RpcError>, ClientError>;
    fn get_tx_out_set_info(&self) -> Result<Result<TxOutSetInfo, RpcError>, ClientError>;

    fn minerids(&self, height: String) -> Result<Result<MinerIDs, RpcError>, ClientError>;
    fn notaries(&self, height: String) -> Result<Result<Notaries, RpcError>, ClientError>;

    fn get_info(&self) -> Result<Result<Info, RpcError>, ClientError>;


    fn backup_wallet(&self, file_name: &str) -> Result<Result<String, RpcError>, ClientError>;
    fn dump_privkey(&self, address: &str) -> Result<Result<String, RpcError>, ClientError>;

    fn get_new_address(&self) -> Result<Result<String, RpcError>, ClientError>;

    fn get_transaction(&self, tx: &TransactionId) -> Result<Result<Transaction, RpcError>, ClientError>;
}