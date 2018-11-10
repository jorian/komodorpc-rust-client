// this needs to eventually fetch data from official sources, like Notary Nodes
use core::fmt;

#[derive(Debug)]
pub enum Assetchain {
    REVS            = 10196,
    SUPERNET        = 11341,
    DEX             = 11890,
    PANGEA          = 14068,
    JUMBLR          = 15106,
    BET             = 14250,
    CRYPTO          = 8516,
    HODL            = 14431,
    MSHARK          = 8846,
    BOTS            = 11964,
    MGW             = 12386,
    COQUI           = 14276,
    WLC             = 12167,
    KV              = 8299,
    CEAL            = 11116,
    MESH            = 9455,
    MNZ             = 14337,
    AXO             = 12927,
    ETOMIC          = 10271,
    BTCH            = 8800,
    PIZZA           = 11608,
    BEER            = 8923,
    NINJA           = 8427,
    OOT             = 12467,
    BNTN            = 14358,
    CHAIN           = 15587,
    PRLPAY          = 9679,
    DSEC            = 11557,
    GLXT            = 13109,
    EQL             = 10306,
    ZILLA           = 10041,
    RFOX            = 32269,
    SEC             = 11540,
    CCL             = 20849,
    PIRATE          = 45453,
    MGNX            = 20731,
    PGT             = 46705,
    KMDICE          = 30177,
    DION            = 23895,
}

impl fmt::Display for Assetchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         fmt::Debug::fmt(self, f)
    }
}