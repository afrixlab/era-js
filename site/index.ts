import * as era from 'era-js';
import axios from 'axios';
import { CreateWalletRequest } from './types';
//@ts-ignore
import { Buffer } from 'buffer';

async function main(){
    let project_id: string = '2764d2e8-ff92-49d5-ac95-f5bbad1d95b8';
    let param: CreateWalletRequest = {
        keep_copy: true,
        identifier: "219678097099021",
        pin: "222333",
        recovery_password: "password1234567"
    }
    let sdk = new EraSDK(project_id);
    // let response = await sdk.createWallet(param);
    // console.log(response);
    let result = await sdk.getWallet(param.identifier, true);
    let base_wallet = new era.BaseWallet({
        project_shard: result.shards[0],
        system_shard: result.shards[1]
    });
    console.log(result.seed_hash);

    let key: era.Signer = base_wallet.build(param.pin);
    console.log(key.as_mnemonic());

    console.log(base_wallet.verify_key(result.seed_hash));

    let polkadot_signer: era.PolkadotSigner = key.to_polkadot_signer();
    console.log(polkadot_signer.fetch_key())
    let message = Buffer.from([62, 75, 66, 66, 65, 72]);
    let signature = polkadot_signer.sign_transaction(message)
    console.log(signature)
}


class EraSDK {
    private project_id: string;

    constructor(project_id: string){
        this.project_id = project_id;
    }

    async createWallet(param: CreateWalletRequest): Promise<any> {
        param.project_id = this.project_id;
        try {
            const response = axios.post('http://127.0.0.1:8080/wallet/create', param)
            return (await response).data;
        }catch(error) {
            console.log(error);
            throw error;
        }        
    }

    async getWallet(identifier: string, full_copy: boolean = false): Promise<any> {
           try {
            const response = axios.post('http://127.0.0.1:8080/wallet/get', {
                identifier,
                full_copy,
                project_id: this.project_id
            })
            return (await response).data;
        }catch(error) {
            console.log(error);
            throw error;
        } 
    }

}

function decodeBase64(base64String: string): Uint8Array {
   const buffer = Buffer.from(base64String, 'base64');
    const uint8Array = new Uint8Array(buffer);
    return uint8Array
}




main().then().catch((err)=> {
    console.error(err);
});
