use jsonrpc_client::ClientError;
use jsonrpc_client::RpcError;
use TransactionId;
use Transaction;
use Info;
use BlockHash;

pub trait KomodoRpcApi {
    // Komodo has a large set of RPC calls
    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, ClientError>;

    fn get_info(&self) -> Result<Result<Info, RpcError>, ClientError>;

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError>;
}