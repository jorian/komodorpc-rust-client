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
- [ ]   find redeemScript in previous transactions (if any)
- [ ]   make sensible use of references in arguments
- [ ]   use `Into<String>` as argument for String types
 

This wrapper aims to:
- be stateless. It's up to the app using this library to maintain state.

### Currently supported calls

Below is a list of RPC calls in this Rust client as of KMDversion 0.3.3b

#### Notes

- `valueSat` and `valueZat` are omitted from `getrawtransaction` and `decoderawtransaction`, respectively
- Several network RPCs return no response from komodod. Tempfix: An RpcError with code 0 is returned.

#### RPCs

##### FSM 
- [ ]    FSMaddress [pubkey]
- [ ]    FSMcreate name states
- [ ]    FSMinfo fundingtxid
- [ ]    FSMlist

##### Addressindex
- [x]    getaddressbalance
- [x]    getaddressdeltas
- [x]    getaddressmempool
- [x]    getaddresstxids
- [x]    getaddressutxos
- [x]    getsnapshot

##### Auction 
- [ ]    auctionaddress [pubkey]

##### Blockchain
- [x]    coinsupply <height>
- [x]    getbestblockhash
- [x]    getblock "hash|height" ( verbose )
- [x]    getblockchaininfo
- [x]    getblockcount
- [x]    getwalletinfo
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
- [x]    getinfo
- [ ]    help ( "command" )
- [ ]    stop

##### Crosschain
- [ ]    MoMoMdata symbol kmdheight ccid
- [ ]    assetchainproof needs a txid
- [ ]    calc_MoM height MoMdepth
- [ ]    getNotarisationsForBlock blockHash
- [ ]    height_MoM height
- [ ]    migrate_completeimporttransaction importTx
- [ ]    migrate_converttoexport rawTx dest_symbol export_amount
- [ ]    migrate_createimporttransaction burnTx payouts
- [ ]    scanNotarisationsDB blockHeight symbol [blocksLimit=1440]

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
- [ ]    gatewaysinfo bindtxid
- [ ]    gatewayslist
- [ ]    gatewaysmarkdone completesigningtx cointxid
- [ ]    gatewaysmultisig txidaddr
- [ ]    gatewayspartialsign txidaddr refcoin hex
- [ ]    gatewayspending bindtxid coin
- [ ]    gatewaysprocessed bindtxid coin
- [ ]    gatewayswithdraw bindtxid coin withdrawpub amount

##### Generating
- [ ]    generate numblocks
- [ ]    getgenerate
- [ ]    setgenerate generate ( genproclimit )

##### Heir
- [ ]    heiraddress func txid amount [destpubkey]

##### Lotto 
- [ ]    lottoaddress [pubkey]

##### Marmara
- [ ]    marmaraadress [pubkey]

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
- [ ]    oraclesinfo oracletxid
- [ ]    oracleslist
- [ ]    oraclesregister oracletxid datafee
- [ ]    oraclessamples oracletxid batonutxo num
- [ ]    oraclessubscribe oracletxid publisher amount

##### Payments 
- [ ]    paymentsaddress [pubkey]

##### Pegs 
- [ ]    pegssaddress [pubkey]

##### Prices 
- [ ]    pricesaddfunding fundingtxid bettoken amount
- [ ]    pricesaddress [pubkey]
- [ ]    pricesbet fundingtxid bettoken amount leverage
- [ ]    pricescreate bettoken oracletxid margin mode longtoken shorttoken maxleverage funding N [pubkeys]
- [ ]    pricesfinish fundingtxid bettoken bettxid
- [ ]    pricesinfo fundingtxid
- [ ]    priceslist
- [ ]    pricesstatus fundingtxid bettoken bettxid

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
- [ ]    tokenaddress [pubkey]
- [ ]    tokenask numtokens tokenid price
- [ ]    tokenbalance tokenid [pubkey]
- [ ]    tokenbid numtokens tokenid price
- [ ]    tokencancelask tokenid asktxid
- [ ]    tokencancelbid tokenid bidtxid
- [ ]    tokenconvert evalcode tokenid pubkey amount
- [ ]    tokencreate name supply description
- [ ]    tokenfillask tokenid asktxid fillunits
- [ ]    tokenfillbid tokenid bidtxid fillamount
- [ ]    tokeninfo tokenid
- [ ]    tokenlist
- [ ]    tokenorders [tokenid]
- [ ]    tokentransfer tokenid destpubkey amount

##### Util
- [ ]    createmultisig nrequired ["key",...]
- [ ]    decodeccopret hex
- [ ]    estimatefee nblocks
- [ ]    estimatepriority nblocks
- [ ]    invalidateblock "hash"
- [ ]    jumblr_deposit "depositaddress"
- [ ]    jumblr_pause
- [ ]    jumblr_resume
- [ ]    jumblr_secret "secretaddress"
- [ ]    reconsiderblock "hash"
- [ ]    txnotarizedconfirmed txid
- [ ]    validateaddress "komodoaddress"
- [ ]    verifymessage "komodoaddress" "signature" "message"
- [ ]    z_validateaddress "zaddr"

##### Wallet 
- [ ]    addmultisigaddress nrequired ["key",...] ( "account" )
- [x]    backupwallet "destination" (*requires `-exportdir` to be set*)
- [x]    dumpprivkey "komodoaddress" 
- [x]    dumpwallet "filename" (*requires `-exportdir` to be set*)
- [ ]    encryptwallet "passphrase"
- [ ]    getaccount "KMD_address" (*deprecated*)
- [ ]    getaccountaddress "account" (*deprecated*)
- [ ]    getaddressesbyaccount "account" (*deprecated*)
- [x]    getbalance ( "account" minconf includeWatchonly ) (*account deprecated*)
- [x]    getnewaddress ( "account" ) (*account deprecated*)
- [ ]    getrawchangeaddress
- [ ]    getreceivedbyaccount "account" ( minconf ) (*account deprecated*)
- [ ]    getreceivedbyaddress "KMD_address" ( minconf )
- [x]    gettransaction "txid" ( includeWatchonly )
- [ ]    getunconfirmedbalance
- [x]    getwalletinfo
- [ ]    importaddress "address" ( "label" rescan )
- [ ]    importprivkey "komodoprivkey" ( "label" rescan )
- [ ]    importwallet "filename"
- [ ]    keypoolrefill ( newsize )
- [ ]    listaccounts ( minconf includeWatchonly)
- [ ]    listaddressgroupings
- [ ]    listlockunspent
- [ ]    listreceivedbyaccount ( minconf includeempty includeWatchonly)
- [ ]    listreceivedbyaddress ( minconf includeempty includeWatchonly)
- [ ]    listsinceblock ( "blockhash" target-confirmations includeWatchonly)
- [ ]    listtransactions ( "account" count from includeWatchonly)
- [ ]    listunspent ( minconf maxconf  ["address",...] )
- [ ]    lockunspent unlock [{"txid":"txid","vout":n},...]
- [ ]    move "fromaccount" "toaccount" amount ( minconf "comment" )
- [ ]    resendwallettransactions
- [ ]    sendfrom "fromaccount" "toKMDaddress" amount ( minconf "comment" "comment-to" )
- [ ]    sendmany "fromaccount" {"address":amount,...} ( minconf "comment" ["address",...] )
- [ ]    sendtoaddress "KMD_address" amount ( "comment" "comment-to" subtractfeefromamount )
- [ ]    setaccount "KMD_address" "account" (*deprecated*)
- [ ]    setpubkey pubkey
- [ ]    settxfee amount
- [ ]    signmessage "t-addr" "message"
- [ ]    z_exportkey "zaddr"
- [ ]    z_exportviewingkey "zaddr"
- [ ]    z_exportwallet "filename"
- [ ]    z_getbalance "address" ( minconf )
- [ ]    z_getnewaddress ( type )
- [ ]    z_getoperationresult (["operationid", ... ])
- [ ]    z_getoperationstatus (["operationid", ... ])
- [ ]    z_gettotalbalance ( minconf includeWatchonly )
- [ ]    z_importkey "zkey" ( rescan startHeight )
- [ ]    z_importviewingkey "vkey" ( rescan startHeight )
- [ ]    z_importwallet "filename"
- [ ]    z_listaddresses ( includeWatchonly )
- [ ]    z_listoperationids
- [ ]    z_listreceivedbyaddress "address" ( minconf )
- [ ]    z_mergetoaddress ["fromaddress", ... ] "toaddress" ( fee ) ( transparent_limit ) ( shielded_limit ) ( memo )
- [ ]    z_sendmany "fromaddress" [{"address":... ,"amount":...},...] ( minconf ) ( fee )
- [ ]    z_shieldcoinbase "fromaddress" "tozaddress" ( fee ) ( limit )
- [ ]    zcbenchmark benchmarktype samplecount
- [ ]    zcrawjoinsplit rawtx inputs outputs vpub_old vpub_new
- [ ]    zcrawkeygen
- [ ]    zcrawreceive zcsecretkey encryptednote
- [ ]    zcsamplejoinsplit