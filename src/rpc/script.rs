use Address;

#[derive(Deserialize, Serialize, Debug)]
pub struct DecodedScript {
    pub asm: String,
    pub reqSigs: Option<u16>,
    #[serde(rename = "type")]
    pub script_type: ScriptType,
    pub addresses: Vec<Option<Address>>,
    pub p2sh: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum ScriptType {
    #[serde(rename = "pubkey")]
    PubKey,
    #[serde(rename = "pubkeyhash")]
    PubKeyHash,
    #[serde(rename = "multisig")]
    MultiSig,
    #[serde(rename = "nonstandard")]
    NonStandard,
    #[serde(rename = "scripthash")]
    ScriptHash,
    #[serde(rename = "witness_v0_keyhash")]
    WitnessPubKeyHash,
    #[serde(rename = "witness_unknown")]
    WitnessUnknown,
    /// Appears for generated transactions
    #[serde(rename = "nulldata")]
    NullData,
}

