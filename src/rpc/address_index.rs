use Address;

#[derive(Debug, Deserialize)]
pub struct AddressBalance {
    pub balance: u64,
    pub received: u64,
}

#[derive(Debug, Deserialize)]
pub struct AddressDeltas(Vec<AddressDelta>);


#[derive(Debug, Deserialize)]
pub struct AddressDelta {
    pub satoshis: i64,
    pub txid: String,
    pub index: u32,
    pub blockindex: u64,
    pub height: u64,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct AddressMempool(Vec<AddressMempoolDelta>);

#[derive(Debug, Deserialize)]
pub struct AddressMempoolDelta {
    address: String,
    txid: String,
    index: u32,
    satoshis: i64,
    timestamp: u64,
    prevtxid: String,
    prevout: u32,
}

#[derive(Debug, Deserialize)]
pub struct AddressTxIDs(Vec<String>); // todo: a vec with txids, needs work.

#[derive(Debug, Deserialize)]
pub struct AddressUtxos {
    pub vec: Vec<AddressUtxo>
}

#[derive(Debug, Deserialize)]
pub struct AddressUtxo {
    pub address: String,
    pub txid: String,
    pub height: u64,
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    pub script: String,
    pub satoshis: u64 // output always positive, no signing needed
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub addresses: Vec<Address>,
    pub total: f64,
    pub average: f64,
    pub utxos: u64,
    pub total_addresses: u64,
    pub start_height: u64,
    pub ending_height: u64,
    pub start_time: u64,
    pub end_time: u64,
}


/*
//    getaddressbalance
//    getaddressdeltas
//    getaddressmempool
//    getaddresstxids
//    getaddressutxos
//    getsnapshot

getaddressbalance '{"addresses":["Rxxx"]}'
{
  "balance": 70015354651,
  "received": 48815719160525
}

// Returns all changes for an address
getaddressdeltas '{"addresses":["Rxxx"]}'
[
    ...
    {
        "satoshis": 2500064992,
        "txid": "2f8a94c102ddb6ffc8fc8e1d4e212f93b9b33afa590872edb072a55bd47db5c4",
        "index": 0,
        "blockindex": 0,
        "height": 28545,
        "address": "Rxxx"
    }
    ...
]

getaddressmempool '{"addresses":["Rxxx"]}'
[
  {
    "address"  (string) The base58check encoded address
    "txid"  (string) The related txid
    "index"  (number) The related input or output index
    "satoshis"  (number) The difference of satoshis
    "timestamp"  (number) The time the transaction entered the mempool (seconds)
    "prevtxid"  (string) The previous txid (if spending)
    "prevout"  (string) The previous transaction output index (if spending)
  }
]

getaddresstxids '{"addresses":["Rxxx"]}'
[
  "transactionid"  (string) The transaction id
  ,...
]

getaddressutxos '{"addresses":["Rxxx"]}'
[
  {
    "address"  (string) The address base58check encoded
    "txid"  (string) The output txid
    "height"  (number) The block height
    "outputIndex"  (number) The output index
    "script"  (strin) The script hex encoded
    "satoshis"  (number) The number of satoshis of the output
  }
]

getsnapshot (top)
{
   "addresses": [
    {
      "addr": "address",
      "amount": "100.0"
    },
    {
      "addr": "address",
      "amount": "23.45"
    }
  ],
  "total": 123.45           (numeric) Total amount in snapshot
  "average": 61.7,          (numeric) Average amount in each address
  "utxos": 14,              (number) Total number of UTXOs in snapshot
  "total_addresses": 2,     (number) Total number of addresses in snapshot,
  "start_height": 91,       (number) Block height snapshot began
  "ending_height": 91       (number) Block height snapsho finished,
  "start_time": 1531982752, (number) Unix epoch time snapshot started
  "end_time": 1531982752    (number) Unix epoch time snapshot finished
}
*/