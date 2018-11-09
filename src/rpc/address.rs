//pub enum Payload {
//    Pubkey(String),
//    PubkeyHash(String),
//    ScriptHash(String)
//}
//
// TODO: need to properly deserialize info from daemon
//// TODO: need to read up on deserializing with Serde
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct Address {
//    // the bitcoin library uses a payload type, which determines p2ph, p2pkh, scripthash or segwit address.
//    // Komodo doesn't use segwit, so we can skip that one.
//
//    // TODO: for now use String
//    // pub payload: Payload
//    pub payload: String
//}
//
//impl Address {
//    // this method should take a reference to a pubkey to calculate the correct base58 KMD address
//    pub fn p2pkh() -> Address {
//        unimplemented!();
//    }
//
//    // TODO: support for uncompressed?
//
//    // TODO: support for p2pk?
//    // seems to be early day addresses
//
//    // this method takes a script, and calculates an address based on this script:
//    pub fn p2sh() -> Address {
//        unimplemented!();
//    }
//}

pub enum KomodoAddress {
    Transparent (TAddress),
    Shielded (ZAddress)
}

pub struct TAddress {

}

pub struct ZAddress {

}