const { Account, KeyLanguage, KeyLength, accountFromMnemonic } = require('era-js');


function main(){
    let seed = new Account(KeyLength.Words24, KeyLanguage.Spanish);
    console.log(seed.as_mnemonic())

    let mnemonic = accountFromMnemonic("right pave sketch blanket across oppose route shell favorite domain comfort super");
    console.log(mnemonic.as_hex())

    let root_key = mnemonic.derive_root_key();
    console.log(root_key)

    let path = "m/44'/60'/0'/0/0";
    let key = mnemonic.derive_extended_key(path);
    console.log(key)

}

main()