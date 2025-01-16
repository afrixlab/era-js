use super::{Mnemonic,Seed,Language,MnemonicType};
use super::{wasm_bindgen, JsValue, Serialize, encode};




#[wasm_bindgen(js_name = Seed)]
#[derive(Serialize)]
pub struct JsSeed {
    seed: Vec<u8>,
    mnemonic: String,
}

#[wasm_bindgen(js_class = Seed)]
impl JsSeed {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsValue, JsValue> {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let seed = Seed::new(&mnemonic, "");
        let value = serde_wasm_bindgen::to_value(&JsSeed {
            seed: seed.as_bytes().to_vec(),
            mnemonic: mnemonic.phrase().to_string(),
        })?;
        Ok(value)
    }
    #[wasm_bindgen(getter)]
    pub fn as_bytes(&self) -> Vec<u8> {
        self.seed.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn as_hex(&self) -> String {
        encode(&self.seed)
    }
    #[wasm_bindgen(getter)]
    pub fn as_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    #[wasm_bindgen(js_name = fromStr)]
    pub fn from_str(str: &str) -> JsSeed {
        let seed = Seed::new(
            &Mnemonic::from_phrase(str, bip39::Language::English).unwrap(),
            "",
        );
        JsSeed {
            seed: seed.as_bytes().to_vec(),
            mnemonic: str.to_string(),
        }
    }
    #[wasm_bindgen(js_name = fromMnemonic)]
    pub fn from_mnemonic(mnemonic: &str) -> JsSeed {
        let seed = Seed::new(
            &Mnemonic::from_phrase(mnemonic, bip39::Language::English).unwrap(),
            "",
        );
        JsSeed {
            seed: seed.as_bytes().to_vec(),
            mnemonic: mnemonic.to_string(),
        }
    }
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.seed.clone()
    }
    #[wasm_bindgen(js_name = toStr)]
    pub fn to_str(&self) -> String {
        self.mnemonic.clone()
    }
    #[wasm_bindgen(js_name = toMnemonic)]
    pub fn to_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }
}