use crate::wasm_bindgen;
use crate::{Serialize,Deserialize};
use sp_core::sr25519::Signature;
use sp_core::{sr25519,crypto::Ss58Codec, Pair};
use crate::to_value;
use crate::JsValue;

#[derive(Serialize,Deserialize)]
pub struct KeyObject {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
    pub path: String,
}

#[wasm_bindgen]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolkadotSigner {
    signer: Vec<u8>,
    path: String
}

#[wasm_bindgen]
impl PolkadotSigner {
    
    #[wasm_bindgen]
    pub fn fetch_key(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.get_key()).unwrap())
    }
}


impl PolkadotSigner {
    pub fn new(signer: String, path: String) -> Self {
        let signer = hex::decode(signer.trim_start_matches("0x")).unwrap();
        Self { signer, path }
    }

    fn get_key(&self) -> KeyObject {
        let pair = sr25519::Pair::from_seed_slice(&self.signer).unwrap();
        // Extract public and private keys
        let public = pair.public();
        let private = pair.to_raw_vec();

        // SS58 address (default network prefix 42 for Substrate generic)
        let address = public.to_ss58check();
        KeyObject {
            private_key: format!("0x{}",hex::encode(private)),
            public_key: format!("0x{}",hex::encode(public)),
            address,
            path: self.path.clone(),   
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let pair = sr25519::Pair::from_seed_slice(&self.signer).unwrap();
        pair.sign(message)
    }
        
}
