# Komodo (KMD) RPC client
A RPC client for [Komodo](https://komodoplatform.com) (KMD), made in Rust. In the future, you can use this library
to create Rust apps that communicate with Komodo blockchains. Since Crypto Conditions is supported in all
Komodo assetchains, dApps can be created through this crate. Or at least, that is my goal for this.

With credits to the coblox team's [Rust BTC RPC client](https://github.com/coblox/bitcoinrpc-rust-client).

This is a work in progress! 

todo:
- [ ]   implement all API calls
- [ ]   argument Address into type
- [x]   handle empty responses (setaccount)
- [x]   improve error handling Client
- [ ]   ~~find redeemScript in previous transactions (if any)~~
- [ ]   make sensible use of references in arguments
- [ ]   use `Into<String>` as argument for String types
- [ ]   get rid of needing AddressList for single address in parameter
- [x]   define own `Chain` if needed assetchain isn't in enum
- [ ]   properly expose arguments / types
- [x]   handle special case: wildcard `"*"` in `fromaddresses`, `z_mergetoaddress`
- [ ]   supply Client with manual RPC credentials
- [ ]   struct Unspent contains account in response, which is deprecated. Need to circumvent
- [ ]   `listaddressgroupings` returns an array with 3 different types, for each distinct address. how to fix in serde
- [ ]   add helper functions to several types
    - a selection of utxos upon entering an amount to `address_utxos`

This wrapper aims to:
- be stateless, i.e. a thin layer. It's up to the app using this library to maintain state.

### Features
- Convert an utxolist retrieved through `getaddressutxos` to inputs in `createrawtransaction`
- Automatically fetches KMD / Assetchain parameters from config file on Win, MacOS and Ubuntu/Debian

### Currently supported calls

Below is a list of RPC calls in this Rust client as of KMDversion 0.3.3b

#### Notes

- `valueSat` and `valueZat` are omitted from `getrawtransaction` and `decoderawtransaction`, respectively
- Several network RPCs return no response from komodod. These RPCs are unsupported until further notice, being:
    - addnode
    - clearbanned
    - z_importkey
    - z_importviewingkey
    - z_importwallet
    - importaddress
    - importwallet
- Sapling does not support `z_exportviewingkey`, or viewing keys in general, yet
- RPC arguments checked before actual request:
    - address in address parameter is valid (basic length check for now)
- `sendtoaddress` returns a transaction id that is little-endian, which is reversed from what you see on the explorer. Call `be_hex_string()` on the `TransactionId` type and you get the big-endian txid.
    - The transaction id that is deserialized will always be little-endian.

#### RPCs

##### CClib
- [ ]    cclib method [evalcode] [JSON params]
- [ ]    cclibaddress [evalcode] [pubkey]
- [ ]    cclibinfo

##### FSM 
- [ ]    FSMaddress [pubkey]
- [ ]    FSMcreate name states
- [ ]    FSMinfo fundingtxid
- [ ]    FSMlist

##### Addressindex
- [ ]    checknotarization
- [x]    getaddressbalance
- [x]    getaddressdeltas
- [x]    getaddressmempool
- [x]    getaddresstxids
- [x]    getaddressutxos
- [ ]    getnotarypayinfo
- [x]    getsnapshot

##### Auction 
- [ ]    auctionaddress [pubkey]

##### Blockchain
- [x]    coinsupply <height>
- [x]    getbestblockhash
- [x]    getblock "hash|height" ( verbose )
- [x]    getblockchaininfo
- [x]    getblockcount
- [x]    getblockhash index
- [ ]    getblockhashes timestamp (*requires timestampindex=1*)
- [x]    getblockheader "hash" ( verbose )
- [x]    getchaintips
- [x]    getdifficulty
- [ ]    getlastsegidstakes depth
- [x]    getmempoolinfo
- [x]    getrawmempool ( verbose ) //
- [ ]    getspentinfo "txid" index
- [x]    gettxout "txid" n ( includemempool )
- [x]    gettxoutproof ["txid",...] ( blockhash ) needs -txindex
- [x]    gettxoutsetinfo
- [ ]    kvsearch key
- [ ]    kvupdate key "value" days passphrase
- [x]    minerids needs height 
- [x]    notaries height timestamp
- [ ]    verifychain ( checklevel numblocks )
- [ ]    verifytxoutproof "proof"

##### Channels
- [ ]    channelsaddress destpubkey
- [ ]    channelsclose opentxid
- [ ]    channelsinfo [opentxid]
- [ ]    channelsinfo [opentxid]
- [ ]    channelsopen destpubkey numpayments payment
- [ ]    channelspayment prevtxid origtxid n amount
- [ ]    channelsrefund stoptxid origtxid

##### Control
- [ ]    geterablockheights
- [ ]    getiguanajson
- [x]    getinfo
- [ ]    getnotarysendmany
- [ ]    help ( "command" )
- [ ]    stop

##### Crosschain
- [ ]    MoMoMdata symbol kmdheight ccid
- [ ]    assetchainproof needs a txid
- [ ]    calc_MoM height MoMdepth
- [ ]    getNotarisationsForBlock blockHash
- [ ]    getimports "hash|height"
- [ ]    getwalletburntransactions "count"
- [ ]    height_MoM height
- [ ]    importdual only works on -ac_import chains
- [ ]    importgatewaybind only works on -ac_import chains
- [ ]    importgatewaycompletesigning only works on -ac_import chains
- [ ]    importgatewayddress [pubkey]
- [ ]    importgatewaydeposit only works on -ac_import chains
- [ ]    importgatewaybind only works on -ac_import chains
- [ ]    importgatewaymarkdone completesigningtx coin
- [ ]    importgatewayspartialsign only works on -ac_import chains
- [ ]    importgatewaypendingdeposits bindtxid coin
- [ ]    importgatewaypendingwithdraws bindtxid coin
- [ ]    importgatewayprocessed bindtxid coin
- [ ]    importgatewaywithdraw only works on -ac_import chains
- [ ]    migrate_checkburntransactionsource burntxid
- [ ]    migrate_completeimporttransaction importTx
- [ ]    migrate_converttoexport rawTx dest_symbol export_amount
- [ ]    migrate_createburntransaction dest_symbol dest_addr amount [tokenid]
- [ ]    migrate_createimporttransaction burnTx payouts [notarytxid-1]..[notarytxid-N]
- [ ]    migrate_createnotaryapprovaltransaction burntxid txoutproof
- [ ]    scanNotarisationsDB blockHeight symbol [blocksLimit=1440]
- [ ]    selfimport only works on -ac_import chains

##### Dice
- [ ]    diceaddfunds name fundingtxid amount
- [ ]    diceaddress [pubkey]
- [ ]    dicebet name fundingtxid amount odds
- [ ]    dicefinish name fundingtxid bettxid
- [ ]    dicefund name funds minbet maxbet maxodds timeoutblocks
- [ ]    diceinfo fundingtxid
- [ ]    dicelist
- [ ]    dicestatus name fundingtxid bettxid

##### Disclosure 
- [ ]    z_getpaymentdisclosure "txid" "js_index" "output_index" ("message")
- [ ]    z_validatepaymentdisclosure "paymentdisclosure"

##### Faucet 
- [ ]    faucetaddress [pubkey]
- [ ]    faucetfund amount
- [ ]    faucetget
- [ ]    faucetinfo

##### Gateways 
- [ ]    gatewaysaddress [pubkey]
- [ ]    gatewaysbind tokenid oracletxid coin tokensupply M N pubkey(s)
- [ ]    gatewaysclaim bindtxid coin deposittxid destpub amount
- [ ]    gatewayscompletesigning withdrawtxid coin hex
- [ ]    gatewaysdeposit bindtxid height coin cointxid claimvout deposithex proof destpub amount
- [ ]    gatewaysdumpprivkey bindtxid address
- [ ]    gatewaysexternaladdress bindtxid pubkey
- [ ]    gatewaysinfo bindtxid
- [ ]    gatewayslist
- [ ]    gatewaysmarkdone completesigningtx cointxid
- [ ]    gatewayspartialsign txidaddr refcoin hex
- [ ]    gatewayspendingdeposits bindtxid coin
- [ ]    gatewayspendingwithdraws bindtxid coin
- [ ]    gatewaysprocessed bindtxid coin
- [ ]    gatewayswithdraw bindtxid coin withdrawpub amount

##### Generating
- [ ]    generate numblocks
- [ ]    getgenerate
- [ ]    setgenerate generate ( genproclimit )

##### Heir
- [ ]    heiradd txfee funds fundingtxid
- [ ]    heiraddress pubkey
- [ ]    heirclaim txfee funds fundingtxid
- [ ]    heirfund txfee funds heirname heirpubkey inactivitytime memo [tokenid]
- [ ]    heirinfo fundingtxid
- [ ]    heirlist

##### Lotto 
- [ ]    lottoaddress [pubkey]

##### Marmara
- [ ]    Marmaraaddress [pubkey]
- [ ]    marmaracreditloop txid
- [ ]    marmarainfo firstheight lastheight minamount maxamount [currency issuerpk]
- [ ]    marmaraissue receiverpk amount currency matures approvaltxid
- [ ]    marmaralock amount unlockht
- [ ]    marmarapoolpayout perc firstheight "[[\"pubkey\":shares], ...]"
- [ ]    marmarareceive senderpk amount currency matures batontxid
- [ ]    marmarasettlement batontxid
- [ ]    marmaratransfer receiverpk amount currency matures approvaltxid

##### Mining 
- [x]    getblocksubsidy height
- [x]    getblocktemplate ( "jsonrequestobject" )
- [x]    getlocalsolps
- [x]    getmininginfo
- [x]    getnetworkhashps ( blocks height )
- [x]    getnetworksolps ( blocks height )
- [x]    prioritisetransaction <txid> <priority delta> <fee delta>
- [x]    submitblock "hexdata" ( "jsonparametersobject" )

##### Network
- [x]    addnode "node" "add|remove|onetry" (*empty response*)
- [x]    clearbanned (*empty response*)
- [x]    disconnectnode "node" 
- [x]    getaddednodeinfo dns ( "node" )
- [x]    getconnectioncount
- [x]    getdeprecationinfo
- [x]    getnettotals
- [x]    getnetworkinfo
- [x]    getpeerinfo
- [x]    listbanned
- [x]    ping
- [x]    setban "ip(/netmask)" "add|remove" (bantime) (absolute)

##### Oracles 
- [ ]    oraclesaddress [pubkey]
- [ ]    oraclescreate name description format
- [ ]    oraclesdata oracletxid hexstr
- [ ]    oraclesfund oracletxid
- [ ]    oraclesinfo oracletxid
- [ ]    oracleslist
- [ ]    oraclesregister oracletxid datafee
- [ ]    oraclessamples oracletxid batonutxo num
- [ ]    oraclessubscribe oracletxid publisher amount

##### Payments 
- [ ]    paymentsaddress [pubkey]
- [ ]    paymentsairdrop "[lockedblocks,minamount,mintoaddress,top,bottom,fixedFlag,%22excludeAddress%22,...,%22excludeAddressN%22]"
- [ ]    payments_airdroptokens "[%22tokenid%22,lockedblocks,minamount,mintoaddress,top,bottom,fixedFlag,%22excludePubKey%22,...,%22excludePubKeyN%22]"
- [ ]    paymentscreate "[lockedblocks,minamount,%22paytxid0%22,...,%22paytxidN%22]"
- [ ]    paymentsfund "[%22createtxid%22,amount(,useopret)]"
- [ ]    paymentsinfo "[%22createtxid%22]"
- [ ]    paymentslist
- [ ]    paymentsmerge "[%22createtxid%22]"
- [ ]    paymentsrelease "[%22createtxid%22,amount,(skipminimum)]"
- [ ]    paymentstxidopret "[allocation,%22scriptPubKey%22(,%22destopret%22)]"

##### Pegs 
- [ ]    pegssaddress [pubkey]

##### Prices 
- [ ]    mypriceslist [all|open|closed]
- [ ]    prices maxsamples
- [ ]    pricesaddfunding bettxid amount
- [ ]    pricesaddress [pubkey]
- [ ]    pricesbet amount leverage "synthetic-expression"
- [ ]    pricescashout bettxid
- [ ]    pricesgetorderbook
- [ ]    pricesinfo bettxid [height]
- [ ]    priceslist [all|open|closed]
- [ ]    pricesrefillfund amount
- [ ]    pricesrekt bettxid height
- [ ]    pricessetcostbasis bettxid

##### Rawtransactions 
- [x]    createrawtransaction [{"txid":"id","vout":n},...] {"address":amount,...}
- [x]    decoderawtransaction "hexstring"
- [x]    decodescript "hex"
- [ ]    fundrawtransaction "hexstring"
- [x]    getrawtransaction "txid" ( verbose )
- [x]    sendrawtransaction "hexstring" ( allowhighfees )
- [x]    signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )

##### Rewards 
- [ ]    rewardsaddfunding name fundingtxid amount
- [ ]    rewardsaddress [pubkey]
- [ ]    rewardscreatefunding name amount APR mindays maxdays mindeposit
- [ ]    rewardsinfo fundingtxid
- [ ]    rewardslist
- [ ]    rewardslock name fundingtxid amount
- [ ]    rewardsunlock name fundingtxid [txid]

##### Tokens 
- [ ]    assetsaddress [pubkey]
- [ ]    mytokenorders [evalcode]
- [ ]    tokenaddress [pubkey]
- [ ]    tokenask numtokens tokenid price
- [ ]    tokenbalance tokenid [pubkey]
- [ ]    tokenbid numtokens tokenid price
- [ ]    tokencancelask tokenid asktxid
- [ ]    tokencancelbid tokenid bidtxid
- [ ]    tokenconvert evalcode tokenid pubkey amount
- [ ]    tokencreate name supply description [description] [data]
- [ ]    tokenfillask tokenid asktxid fillunits
- [ ]    tokenfillbid tokenid bidtxid fillamount
- [ ]    tokeninfo tokenid
- [ ]    tokenlist
- [ ]    tokenorders [tokenid]
- [ ]    tokentransfer tokenid destpubkey amount

##### Util
- [x]    createmultisig nrequired ["key",...]
- [ ]    decodeccopret hex
- [ ]    estimatefee nblocks
- [ ]    estimatepriority nblocks
- [ ]    invalidateblock "hash"
- [ ]    ~~jumblr_deposit "depositaddress"~~
- [ ]    ~~jumblr_pause~~
- [ ]    ~~jumblr_resume~~
- [ ]    ~~jumblr_secret "secretaddress"~~
- [ ]    reconsiderblock "hash"
- [ ]    txnotarizedconfirmed txid
- [ ]    validateaddress "komodoaddress"
- [ ]    verifymessage "komodoaddress" "signature" "message"
- [ ]    z_validateaddress "zaddr"

##### Wallet 
- [ ]    addmultisigaddress nrequired ["key",...] ( "account" )
- [x]    backupwallet "destination" (*requires `-exportdir` to be set*)
- [ ]    cleanwallettransactions "txid"
- [x]    dumpprivkey "komodoaddress" 
- [x]    dumpwallet "filename" (*requires `-exportdir` to be set*)
- [ ]    encryptwallet "passphrase"
- [ ]    ~~getaccount "KMD_address"~~ (*deprecated*)
- [ ]    ~~getaccountaddress "account"~~ (*deprecated*)
- [ ]    ~~getaddressesbyaccount "account"~~ (*deprecated*)
- [x]    getbalance ( "account" minconf includeWatchonly ) (*account deprecated*)
- [x]    getnewaddress ( "account" ) (*account deprecated*)
- [x]    getrawchangeaddress
- [ ]    ~~getreceivedbyaccount "account" ( minconf )~~ (*account deprecated*)
- [x]    getreceivedbyaddress "KMD_address" ( minconf )
- [x]    gettransaction "txid" ( includeWatchonly )
- [x]    getunconfirmedbalance
- [x]    getwalletinfo
- [ ]    importaddress "address" ( "label" rescan ) (*empty response*)
- [x]    importprivkey "komodoprivkey" ( "label" rescan ) 
- [ ]    importwallet "filename" (*empty response*)
- [ ]    keypoolrefill ( newsize ) (*empty response*)
- [ ]    ~~listaccounts ( minconf includeWatchonly)~~ (*deprecated*)
- [ ]    listaddressgroupings
- [x]    listlockunspent
- [ ]    ~~listreceivedbyaccount ( minconf includeempty includeWatchonly)~~ (*deprecated*)
- [x]    listreceivedbyaddress ( minconf includeempty includeWatchonly)
- [x]    listsinceblock ( "blockhash" target-confirmations includeWatchonly)
- [x]    listtransactions ( "account" count from includeWatchonly)
- [x]    listunspent ( minconf maxconf  ["address",...] )
- [x]    lockunspent unlock [{"txid":"txid","vout":n},...]
- [ ]    ~~move "fromaccount" "toaccount" amount ( minconf "comment" )~~ (*deprecated*)
- [x]    resendwallettransactions
- [ ]    ~~sendfrom "fromaccount" "toKMDaddress" amount ( minconf "comment" "comment-to" )~~ (*deprecated*)
- [x]    sendmany "fromaccount" {"address":amount,...} ( minconf "comment" ["address",...] )
- [x]    sendtoaddress "KMD_address" amount ( "comment" "comment-to" subtractfeefromamount )
- [ ]    ~~setaccount "KMD_address" "account"~~ (*deprecated*)
- [ ]    ~~setpubkey pubkey~~ (*unsupported due to wrong return type*)
- [x]    settxfee amount
- [x]    signmessage "t-addr" "message"

##### Shielded ([see API](https://zcash.readthedocs.io/en/latest/rtd_pages/payment_api.html))
- [x]    z_exportkey "zaddr"
- [ ]    z_exportviewingkey "zaddr" *unsupported for sapling (https://github.com/zcash/zcash/issues/3060)* 
- [x]    z_exportwallet "filename"
- [x]    z_getbalance "address" ( minconf )
- [x]    z_getnewaddress ( type )
- [x]    z_getoperationresult (["operationid", ... ])
- [x]    z_getoperationstatus (["operationid", ... ])
- [x]    z_gettotalbalance ( minconf includeWatchonly )
- [ ]    z_importkey "zkey" ( rescan startHeight ) (*no output*)
- [ ]    z_importviewingkey "vkey" ( rescan startHeight ) (*no output*)
- [ ]    z_importwallet "filename" (*no output*)
- [x]    z_listaddresses ( includeWatchonly )
- [x]    z_listoperationids
- [x]    z_listreceivedbyaddress "address" ( minconf )
- [x]    z_mergetoaddress ["fromaddress", ... ] "toaddress" ( fee ) ( transparent_limit ) ( shielded_limit ) ( memo )
- [x]    z_sendmany "fromaddress" [{"address":... ,"amount":...},...] ( minconf ) ( fee )
- [x]    z_shieldcoinbase "fromaddress" "tozaddress" ( fee ) ( limit )
- [ ]    ~~zcbenchmark benchmarktype samplecount~~ *deprecated*
- [ ]    ~~zcrawjoinsplit rawtx inputs outputs vpub_old vpub_new~~ *deprecated*
- [ ]    ~~zcrawkeygen~~ *deprecated*
- [ ]    ~~zcrawreceive zcsecretkey encryptednote~~ *deprecated*
- [ ]    ~~zcsamplejoinsplit~~ *deprecated*