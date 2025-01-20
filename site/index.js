const { Account, KeyLanguage,KeyLength, accountFromMnemonic } = require('era-js');


function main(){
    let seed = new Account(KeyLength.Words24, KeyLanguage.Spanish);
    console.log(seed.as_mnemonic())

    let mnemonic = accountFromMnemonic("candy maple cake sugar pudding cream honey rich smooth crumble sweet treat");
    console.log(mnemonic.to_bytes())
}

main()