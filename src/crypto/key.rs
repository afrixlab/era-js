use crate::crypto::crypto::KeyPath;
use crate::to_value;
use crate::Uint8Array;
use crate::{encode, wasm_bindgen, JsValue};
use crate::{Language, Mnemonic, MnemonicType, Seed};
use bip32::Prefix;

/// Represents a BIP-32 account. This object contains a seed and a it's mnemonic.
#[wasm_bindgen]
pub struct Account {
    seed: Vec<u8>,
    mnemonic: String,
}

#[wasm_bindgen]
/// Implementation of the `Account` struct, providing methods for account creation,
/// serialization, and key derivation using BIP-39 and BIP-32 standards.
///
/// # Methods
///
/// - `new(length: KeyLength, lang: KeyLanguage) -> Self`  
///   Creates a new `Account` instance with a generated mnemonic and seed.
///
/// - `as_bytes(&self) -> Uint8Array`  
///   Returns the account seed as a byte array.
///
/// - `as_mnemonic(&self) -> String`  
///   Returns the mnemonic phrase as a string.
///
/// - `as_hex(&self) -> Result<JsValue, JsValue>`  
///   Returns the seed as a hex string prefixed with `0x`.
///
/// - `from_str(str: &str) -> Self`  
///   Creates an `Account` from an existing mnemonic phrase.
///
/// - `to_bytes(&self) -> Vec<u8>`  
///   Returns the account seed as a vector of bytes.
///
/// - `to_str(&self) -> String`  
///   Returns the mnemonic phrase as a string.
///
/// - `to_mnemonic(&self) -> String`  
///   Returns the mnemonic phrase as a string.
///
/// - `derive_root_key(&self) -> String`  
///   Derives the root private key from the seed and returns it as a hex string prefixed with `0x`.
///
/// - `derive_root_public_key(&self) -> String`  
///   Derives the root public key from the seed and returns it as a string in XPUB format.
///
/// - `derive_extended_key(&self, path: &str) -> JsValue`  
///   Derives an extended key from the seed using the provided BIP-32 derivation path and returns it as a JavaScript value.
///
/// All methods are exposed to JavaScript via `wasm_bindgen` for WebAssembly interoperability.
impl Account {
    /// Creates a new Account instance
    ///
    /// # Arguments
    ///
    /// * `length` - An enum containing the length of the mnemonic
    /// * `lang` - The language of the mnemonic
    ///
    /// # Returns
    ///
    /// A new Account instance
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
    /// Converts the account to a byte array
    ///  
    /// # Returns
    ///
    /// The seed as a byte array
    #[wasm_bindgen]
    pub fn as_bytes(&self) -> Uint8Array {
        Uint8Array::from(self.seed.as_slice())
    }

    /// Converts the account to its coresponding mnemonic
    ///  
    /// # Returns
    ///
    /// The mnemonic as a string
    #[wasm_bindgen]
    pub fn as_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    /// Converts the account seed to a hex string
    ///  
    /// # Returns
    ///
    /// The seed as a hex string. The seed is prefixed with `0x`
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

    /// Converts the account to a byte array
    ///  
    /// # Returns
    ///
    /// The seed as a byte array
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.seed.clone()
    }

    /// Converts the account to its coresponding mnemonic
    ///  
    /// # Returns
    ///
    /// The mnemonic as a string
    #[wasm_bindgen]
    pub fn to_str(&self) -> String {
        self.mnemonic.clone()
    }

    /// Converts the account to its coresponding mnemonic
    ///  
    /// # Returns
    ///
    /// The mnemonic as a string
    #[wasm_bindgen]
    pub fn to_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    /// Derives the root private key from the seed. This is the `m` path in the BIP-32 derivation path
    ///  
    /// # Returns
    ///
    /// The root private key as a hex string. The private key is prefixed with `0x`
    #[wasm_bindgen]
    pub fn derive_root_key(&self) -> String {
        let xpriv = self.generate_root_key();
        format!("0x{}", encode(xpriv.to_bytes()))
    }

    /// Derives the root public key from the seed.
    ///  
    /// # Returns
    ///
    /// The root public key as a hex string.
    #[wasm_bindgen]
    pub fn derive_root_public_key(&self) -> String {
        let xpub = self.generate_root_public_key();
        xpub.to_string(Prefix::XPUB)
    }

    /// Derives an extended key from the seed when given a path.
    /// # Arguments
    ///
    /// * `path` - A String representing the derivation path. The path should be in the BIP-32 format.
    ///  
    /// # Returns
    ///
    /// The root public key as a hex string.
    #[wasm_bindgen]
    pub fn derive_extended_key(&self, path: &str) -> JsValue {
        let key_object = self.generate_extended_key(path);
        to_value(&key_object).unwrap()
    }
}

/// Derives an account from a mnemonic.
/// # Arguments
///
/// * `mnemonic` - A String of the mnemonic. It can be 12, 15, 18, 21 or 24 words.
///  
/// # Returns
///
/// The Account Object.
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

/// The keyLength enum represents the length of the mnemonic. It can be 12, 15, 18, 21 or 24 words.
#[wasm_bindgen]
pub enum KeyLength {
    Words12,
    Words15,
    Words18,
    Words21,
    Words24,
}
impl From<KeyLength> for MnemonicType {
    /// converts the keyLength enum to the MnemonicType enum
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

/// The keyLanguage enum represents the language of the mnemonic. It can be English, Japanese, Spanish, ChineseSimplified, ChineseTraditional, French, Italian or Korean.
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
    /// converts the keyLanguage enum to the Language enum
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
