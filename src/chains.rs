// this needs to eventually fetch data from official sources, like Notary Nodes
// also it should be able to add your own, manually.
use core::fmt;

#[derive(Debug, Clone, Copy)]
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
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         fmt::Debug::fmt(self, f)
    }
}