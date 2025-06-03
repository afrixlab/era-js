const { 
    Account, KeyLanguage, KeyLength, accountFromMnemonic, 
    encodeShards, decodeShards, BaseWallet, decryptShard, encryptShard 
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

   let e = [
   48,   0,   0,   0,   0,   0,   0,   0,  90, 115, 144,  83,
  136, 253, 154, 197, 166,  37, 161, 252, 168,  23,  21, 213,
  179, 240, 146,  96, 129, 158, 125,  21, 251,  99, 159, 237,
   50,  24,  27, 124, 167,  35,  59,  18, 153, 139, 214,  48,
   49, 237,  12,  64, 242, 198, 172, 133,  96, 176, 195,  42,
   82, 202, 140,  28,  81, 220, 237,  85,  56,  92,  48, 156,
  243, 206,  60,  42, 188,   5, 252, 173, 109, 255, 239,  94,
   58, 192, 237, 241, 219, 145, 208,  88, 223,  78,  97,  13,
  108,  14, 124, 143
] 
    let enc_shard = encryptShard([0,1,2,3], "12345")
    console.log(enc_shard)
    let result = decryptShard(e, "222333");
    console.log(result)

    // let root_key = mnemonic.derive_root_key();
    // console.log(root_key)

    // let path = "m/44'/60'/0'/0/0";
    // let key = mnemonic.derive_extended_key(path);
    // console.log(key)


}

main()