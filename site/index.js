const { 
    Account, KeyLanguage, KeyLength, accountFromMnemonic, 
    encodeShards, decodeShards , encodeShardsTest, decodeShardsTest
    } = require('era-js');

function main(){
    let seed = new Account(KeyLength.Words24, KeyLanguage.Spanish);
    //console.log(seed.as_mnemonic())

    let mnemonic = accountFromMnemonic("right pave sketch blanket across oppose route shell favorite domain comfort super");
    console.log(mnemonic.as_bytes())

    let shards = encodeShards(mnemonic.as_bytes(), 2, 3);
    console.log("shards: ",shards)

    
    // recover in javascript
    let recovered = decodeShards([null, null, null, shards[3], shards[4]],2,3);
    console.log("recoverd: ",recovered)
    
    

    // let root_key = mnemonic.derive_root_key();
    // console.log(root_key)

    // let path = "m/44'/60'/0'/0/0";
    // let key = mnemonic.derive_extended_key(path);
    // console.log(key)

}

main()