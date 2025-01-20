const { Key, KeyLanguage,KeyLength, keyFromMnemonic } = require('era-js');


function main(){
    let seed = new Key(KeyLength.Words24, KeyLanguage.Spanish);
    console.log(seed.as_mnemonic())

    let mnemonic = keyFromMnemonic("candy maple cake sugar pudding cream honey rich smooth crumble sweet treat");
    console.log(mnemonic.to_bytes())
}

main()