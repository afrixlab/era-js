import * as era from 'era-js';
import axios from 'axios';
import { CreateWalletRequest } from './types';
import { Buffer } from 'buffer';
async function main(){
    let project_id: string = '2764d2e8-ff92-49d5-ac95-f5bbad1d95b8';
    let param: CreateWalletRequest = {
        keep_copy: true,
        identifier: "214678097922",
        pin: "222333",
        recovery_password: "password1234567"
    }
    let sdk = new EraSDK(project_id);
    // let result = await sdk.createWallet(param);
    // console.log(result);
    let result = await sdk.getWallet(param.identifier, true);
    let decoded: Uint8Array[] = [];
    for(let i = 0; i < result.shards.length; i++) {
        let res = decodeBase64(result.shards[i]);
        decoded.push(res);
    }
    console.log(decoded[0]);
    let decrypted = era.decryptShard(decoded[0], param.pin)
    console.log(decrypted);
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
