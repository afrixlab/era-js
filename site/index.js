const {Seed} =  require('era-js');


function main(){
    let seed = new Seed();
    console.log(seed.mnemonic);
}

main()