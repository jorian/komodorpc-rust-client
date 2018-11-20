# Komodo (KMD) RPC client
A RPC client for [Komodo](https://komodoplatform.com) (KMD), made in Rust.

This is a work in progress!  

With credits to the coblox team's [Rust BTC RPC client](https://github.com/coblox/bitcoinrpc-rust-client).

This wrapper aims to:
- be stateless. It's up to the app using this library to maintain state.

### Currently supported calls:

##### FSM 
- [ ]    FSMaddress [pubkey]
- [ ]    FSMcreate name states
- [ ]    FSMinfo fundingtxid
- [ ]    FSMlist

##### MofN 
- [ ]    mofnaddress [pubkey]

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
- [ ]    getblock "hash|height" ( verbose )
- [x]    getblockchaininfo
- [x]    getblockcount
- [x]    getwalletinfo
- [x]    getblockhash index
- [ ]    getblockhashes timestamp (*requires timestampindex=1*)
- [x]    getblockheader "hash" ( verbose )
- [x]    getchaintips
- [x]    getdifficulty
- [x]    getmempoolinfo
- [x]    getrawmempool ( verbose ) //
- [ ]    getspentinfo "txid" index
- [ ]    gettxout "txid" n ( includemempool )
- [ ]    gettxoutproof ["txid",...] ( blockhash )
- [x]    gettxoutsetinfo
- [ ]    kvsearch key
- [ ]    kvupdate key "value" days passphrase
- [ ]    minerids needs height 
- [ ]    notaries height timestamp
- [ ]    verifychain ( checklevel numblocks )
- [ ]    verifytxoutproof "proof"

##### Channels
- [ ]    channelsaddress destpubkey
- [ ]    channelscollect paytxid origtxid n amount
- [ ]    channelsinfo
- [ ]    channelsopen destpubkey numpayments payment
- [ ]    channelspayment prevtxid origtxid n amount
- [ ]    channelsrefund stoptxid origtxid
- [ ]    channelsstop destpubkey origtxid

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
- [ ]    gatewaysdeposit bindtxid height coin cointxid claimvout deposithex proof destpub amount
- [ ]    gatewaysinfo bindtxid
- [ ]    gatewayslist
- [ ]    gatewaysmarkdone withdrawtxid coin cointxid
- [ ]    gatewaysmultisig bindtxid coin withtxid txidaddr
- [ ]    gatewayspending bindtxid coin
- [ ]    gatewayswithdraw bindtxid coin withdrawpub amount

##### Generating
- [ ]    generate numblocks
- [ ]    getgenerate
- [ ]    setgenerate generate ( genproclimit )

##### Lotto 
- [ ]    lottoaddress [pubkey]

##### Mining 
- [ ]    getblocksubsidy height
- [ ]    getblocktemplate ( "jsonrequestobject" )
- [ ]    getlocalsolps
- [ ]    getmininginfo
- [ ]    getnetworkhashps ( blocks height )
- [ ]    getnetworksolps ( blocks height )
- [ ]    prioritisetransaction <txid> <priority delta> <fee delta>
- [ ]    submitblock "hexdata" ( "jsonparametersobject" )

##### Network
- [ ]    addnode "node" "add|remove|onetry"
- [ ]    clearbanned
- [ ]    disconnectnode "node"
- [ ]    getaddednodeinfo dns ( "node" )
- [ ]    getconnectioncount
- [ ]    getdeprecationinfo
- [ ]    getnettotals
- [ ]    getnetworkinfo
- [ ]    getpeerinfo
- [ ]    listbanned
- [ ]    ping
- [ ]    setban "ip(/netmask)" "add|remove" (bantime) (absolute)

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
- [ ]    createrawtransaction [{"txid":"id","vout":n},...] {"address":amount,...}
- [ ]    decoderawtransaction "hexstring"
- [ ]    decodescript "hex"
- [ ]    fundrawtransaction "hexstring"
- [ ]    getrawtransaction "txid" ( verbose )
- [ ]    sendrawtransaction "hexstring" ( allowhighfees )
- [ ]    signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )

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

##### Triggers 
- [ ]    triggersaddress [pubkey]

##### Util
- [ ]    createmultisig nrequired ["key",...]
- [ ]    estimatefee nblocks
- [ ]    estimatepriority nblocks
- [ ]    invalidateblock "hash"
- [ ]    jumblr_deposit "depositaddress"
- [ ]    jumblr_pause
- [ ]    jumblr_resume
- [ ]    jumblr_secret "secretaddress"
- [ ]    reconsiderblock "hash"
- [ ]    validateaddress "komodoaddress"
- [ ]    verifymessage "komodoaddress" "signature" "message"
- [ ]    z_validateaddress "zaddr"

##### Wallet 
- [ ]    addmultisigaddress nrequired ["key",...] ( "account" )
- [ ]    backupwallet "destination"
- [x]    dumpprivkey "komodoaddress"
- [ ]    dumpwallet "filename"
- [ ]    encryptwallet "passphrase"
- [ ]    getaccount "KMD_address"
- [ ]    getaccountaddress "account"
- [ ]    getaddressesbyaccount "account"
- [ ]    getbalance ( "account" minconf includeWatchonly )
- [ ]    getbalance64
- [x]    getnewaddress ( "account" ) //todo: are accounts officially supported?
- [ ]    getrawchangeaddress
- [ ]    getreceivedbyaccount "account" ( minconf )
- [ ]    getreceivedbyaddress "KMD_address" ( minconf )
- [x]    gettransaction "txid" ( includeWatchonly )
- [ ]    getunconfirmedbalance
- [ ]    getwalletinfo
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
- [ ]    setaccount "KMD_address" "account"
- [ ]    settxfee amount
- [ ]    signmessage "KMD address" "message"
- [ ]    z_exportkey "zaddr"
- [ ]    z_exportviewingkey "zaddr"
- [ ]    z_exportwallet "filename"
- [ ]    z_getbalance "address" ( minconf )
- [ ]    z_getnewaddress
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