use super::Uint8Array;
use super::{encode, wasm_bindgen, JsValue};
use super::{Language, Mnemonic, MnemonicType, Seed};
use bip32::Prefix;
use serde_wasm_bindgen::to_value;
use crate::crypto::KeyPath;

#[wasm_bindgen]
pub struct Account {
    seed: Vec<u8>,
    mnemonic: String,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    pub fn new(length: KeyLength, lang: KeyLanguage) -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::from(length), Language::from(lang));
        let seed = Seed::new(&mnemonic, "");
        let value = Account {
            seed: seed.as_bytes().to_vec(),
            mnemonic: mnemonic.phrase().to_string(),
        };
        value
    }
    #[wasm_bindgen]
    pub fn as_bytes(&self) -> Uint8Array {
        Uint8Array::from(self.seed.as_slice())
    }

    #[wasm_bindgen]
    pub fn as_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    #[wasm_bindgen]
    pub fn as_hex(&self) -> Result<JsValue, JsValue> {
        let seed = format!("0x{}", encode(&self.seed));
        Ok(to_value(&seed)?)
    }

    #[wasm_bindgen]
    pub fn from_str(str: &str) -> Self {
        let seed = Seed::new(
            &Mnemonic::from_phrase(str, bip39::Language::English).unwrap(),
            "",
        );
        Account {
            seed: seed.as_bytes().to_vec(),
            mnemonic: str.to_string(),
        }
    }

    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.seed.clone()
    }

    #[wasm_bindgen]
    pub fn to_str(&self) -> String {
        self.mnemonic.clone()
    }

    #[wasm_bindgen]
    pub fn to_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    #[wasm_bindgen]
    pub fn derive_root_key(&self) -> String {
        let xpriv = self.generate_root_key();
        hex::encode(xpriv.to_bytes())
    }

    #[wasm_bindgen]
    pub fn derive_root_public_key(&self) -> String {
        let xpub = self.generate_root_public_key();
        xpub.to_string(Prefix::XPUB)
    }
    #[wasm_bindgen]
    pub fn derive_extended_key(&self, path: &str) -> JsValue {
        let key_object = self.generate_extended_key(path);
        to_value(&key_object).unwrap()
    }
}

#[wasm_bindgen(js_name = accountFromMnemonic)]
pub fn from_mnemonic(mnemonic: &str) -> Result<Account, JsValue> {
    let mnemonic = Mnemonic::from_phrase(mnemonic, bip39::Language::English)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let seed = Seed::new(&mnemonic, "");
    Ok(Account {
        seed: seed.as_bytes().to_vec(),
        mnemonic: mnemonic.to_string(),
    })
}

#[wasm_bindgen]
pub enum KeyLength {
    Words12,
    Words15,
    Words18,
    Words21,
    Words24,
}
impl From<KeyLength> for MnemonicType {
    fn from(key_type: KeyLength) -> Self {
        match key_type {
            KeyLength::Words12 => MnemonicType::Words12,
            KeyLength::Words15 => MnemonicType::Words15,
            KeyLength::Words18 => MnemonicType::Words18,
            KeyLength::Words21 => MnemonicType::Words21,
            KeyLength::Words24 => MnemonicType::Words24,
        }
    }
}

#[wasm_bindgen]
pub enum KeyLanguage {
    English,
    Japanese,
    Spanish,
    ChineseSimplified,
    ChineseTraditional,
    French,
    Italian,
    Korean,
}

impl From<KeyLanguage> for Language {
    fn from(key_language: KeyLanguage) -> Self {
        match key_language {
            KeyLanguage::English => Language::English,
            KeyLanguage::Japanese => Language::Japanese,
            KeyLanguage::Spanish => Language::Spanish,
            KeyLanguage::ChineseSimplified => Language::ChineseSimplified,
            KeyLanguage::ChineseTraditional => Language::ChineseTraditional,
            KeyLanguage::French => Language::French,
            KeyLanguage::Italian => Language::Italian,
            KeyLanguage::Korean => Language::Korean,
        }
    }
}
