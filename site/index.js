const { 
    Account, KeyLanguage, KeyLength, accountFromMnemonic, 
    encodeShards, decodeShards, BaseWallet 
    } = require('era-js');

function main(){
    let seed = new Account(KeyLength.Words24, KeyLanguage.Spanish);
    console.log(seed.as_bytes())

    let mnemonic = accountFromMnemonic("right pave sketch blanket across oppose route shell favorite domain comfort super");
    //console.log(mnemonic.as_bytes())

    let shards = encodeShards(seed.as_bytes(), 2, 3);
    //console.log("shards: ",shards)

    
    // recover in javascript
    let recovered = decodeShards([null, null, shards[2], shards[3], shards[4]],2,3);
    //console.log(recovered)


    let wallet_shards = {
        project_shard: shards[2],
        system_shard: shards[3],     
    }
    

    let wallet = new BaseWallet(wallet_shards);
    console.log(wallet)

    let recovery = wallet.reconstruct_shards();
    console.log(recovery)

    
    

    // let root_key = mnemonic.derive_root_key();
    // console.log(root_key)

    // let path = "m/44'/60'/0'/0/0";
    // let key = mnemonic.derive_extended_key(path);
    // console.log(key)

}

main()