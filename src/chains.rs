

use core::fmt;

/// A set of predefined, known assetchains.
/// `Custom(String)` exists to define your own assetchain.
#[derive(Debug, Clone)]
pub enum Chain {
    KMD     ,
    REVS    ,
    SUPERNET,
    DEX     ,
    PANGEA  ,
    JUMBLR  ,
    BET     ,
    CRYPTO  ,
    HODL    ,
    MSHARK  ,
    BOTS    ,
    MGW     ,
    COQUI   ,
    WLC     ,
    KV      ,
    CEAL    ,
    MESH    ,
    MNZ     ,
    AXO     ,
    ETOMIC  ,
    BTCH    ,
    PIZZA   ,
    BEER    ,
    NINJA   ,
    OOT     ,
    BNTN    ,
    CHAIN   ,
    PRLPAY  ,
    DSEC    ,
    GLXT    ,
    EQL     ,
    ZILLA   ,
    RFOX    ,
    SEC     ,
    CCL     ,
    PIRATE  ,
    MGNX    ,
    PGT     ,
    KMDICE  ,
    DION    ,
    DOPE    ,
    Custom(String),
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match self {
             Chain::Custom(name) => fmt::Debug::fmt(name, f),
             _ => fmt::Debug::fmt(self, f)
         }
    }
}