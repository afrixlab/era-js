use bip32::Prefix;
use js_sys::{Error, EvalError};

use crate::chains::polkadot::PolkadotSigner;
use crate::crypto::crypto::KeyPath;
use crate::decrypt;
use crate::{erasure_coding::ErasureError, Deserialize};
use crate::{general_purpose, Engine};
use crate::{wasm_bindgen, JsValue, ReedSolomon};

// base_wallet -> Shares(vec<vec<u8>>) -> Key -> Signer

// base_wallet.build() -> shares
// buildKey(shares) -> Key{key: String}
// key.to_polkadot() -> Signer
// signer.sign()

#[wasm_bindgen]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaseWallet {
    project_shard: Option<Vec<u8>>,
    system_shard: Option<Vec<u8>>,
    recovery_shard: Option<Vec<u8>>,
}

#[wasm_bindgen]
/// Implementation of the `BaseWallet` struct with WebAssembly bindings.
///
/// # Methods
///
/// - `new(value: JsValue) -> Result<BaseWallet, JsValue>`
///   Constructs a new `BaseWallet` instance from a JavaScript value using Serde for deserialization.
///   Returns an error if deserialization fails.
///
/// - `reconstruct_shards(&self) -> Result<JsValue, JsValue>`
///   Reconstructs data shards using Reed-Solomon erasure coding. This method:
///   1. Initializes a Reed-Solomon instance with 2 data shards and 3 parity shards.
///   2. Builds the shards from the wallet data.
///   3. Attempts to reconstruct missing shards, returning an error if reconstruction fails.
///   4. Verifies the integrity of the reconstructed shards.
///   5. Combines only the data shards into a single byte array and returns it as a JavaScript `Uint8Array`.
///   Returns a JavaScript error if any step fails.
impl BaseWallet {
    /// This function takes an object as an arguement and returns a base wallet.
    /// It decodes the base64 encoded shards and builds it into an object.
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Result<BaseWallet, JsValue> {
        #[wasm_bindgen]
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Temp {
            project_shard: Option<String>,
            system_shard: Option<String>,
            recovery_shard: Option<String>,
        }
        let temp: Temp = serde_wasm_bindgen::from_value(value).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to deserialize BaseWallet::expected base64 string, got: {}",
                e
            ))
        })?;

        let project_shard = temp
            .project_shard
            .map(|x| general_purpose::STANDARD.decode(x))
            .transpose()
            .map_err(|e| {
                JsValue::from_str(&format!("Failed to deserialize Project Shard: {}", e))
            })?;
        let system_shard = temp
            .system_shard
            .map(|x| general_purpose::STANDARD.decode(x))
            .transpose()
            .map_err(|e| {
                JsValue::from_str(&format!("Failed to deserialize System Shard: {}", e))
            })?;
        let recovery_shard = temp
            .recovery_shard
            .map(|x| general_purpose::STANDARD.decode(x))
            .transpose()
            .map_err(|e| {
                JsValue::from_str(&format!("Failed to deserialize Recovery Shard: {}", e))
            })?;

        Ok(Self {
            project_shard,
            system_shard,
            recovery_shard,
        })
    }

    #[wasm_bindgen]
    pub fn reconstruct_shards(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.reconstruct_shards_internal()?.into())
    }

    /// Builds a base wallet into a root signer key.
    /// This method assumes that the `BaseWallet` contains atleast the system_shard and one other shard.
    /// The system shard is in a unencrypted but we assume that other shards are in an encrypted format.
    /// The builder automatically builds with the project shard unless told otherwise.
    /// # Arguments
    ///
    /// * `password` - The password to the encrypted shard
    /// * `project_shard` - determines if the builder uses a project shard or not. defaults to `true``
    ///
    /// # Returns
    ///
    /// A Signer: array of the root key.
    /// If the data could not be decoded, an error is returned.
    #[wasm_bindgen]
    pub fn to_signer(
        &mut self,
        password: String,
        project_shard: Option<bool>,
    ) -> Result<Signer, JsValue> {
        Ok(self.build_signer(password, project_shard)?.into())
    }

    #[wasm_bindgen]
    pub fn to_polkadot_signer(
        &mut self,
        password: String,
        project_shard: Option<bool>,
    ) -> Result<PolkadotSigner, JsValue> {
    //) -> Result<u8, JsValue> {

        let signer = self
            .build_signer(password, project_shard)
            .map_err(|e| JsValue::from_str(&format!("Signer Error: {:?}", e)))?;
        let key = signer.generate_extended_key("m/0'");
        let signer = PolkadotSigner::new(key.private_key, "m/0'".into());
        Ok(signer)
    }
}

impl BaseWallet {
    /// Builds the base wallet and returns an aligned data and parity shards
    pub fn build(&self) -> Result<Vec<Option<Vec<u8>>>, Error> {
        match &self {
            BaseWallet {
                project_shard: None,
                system_shard: None,
                ..
            } => return Err(EvalError::new("Fields in BaseWallet are missing").into()),
            BaseWallet {
                system_shard: None,
                recovery_shard: None,
                ..
            } => return Err(EvalError::new("Fields in BaseWallet are missing").into()),
            BaseWallet {
                project_shard: None,
                recovery_shard: None,
                ..
            } => return Err(EvalError::new("Fields in BaseWallet are missing").into()),
            _ => {}
        }
        let shards = vec![
            None,
            None,
            self.project_shard.clone(),
            self.system_shard.clone(),
            self.recovery_shard.clone(),
        ];
        Ok(shards)
    }

    ///  Reconstructs data shards using Reed-Solomon erasure coding.
    pub fn reconstruct_shards_internal(&self) -> Result<Vec<u8>, Error> {
        let reed_solomon = ReedSolomon::new(2, 3).unwrap();
        let mut shards = self.build()?;
        reed_solomon
            .reconstruct(&mut shards)
            .map_err(|_| <ErasureError as Into<JsValue>>::into(ErasureError::FragmentationError))?;
        let shard_refs: Vec<&[u8]> = shards.iter().map(|x| x.as_deref().unwrap()).collect();
        reed_solomon
            .verify(&shard_refs)
            .map_err(|e| JsValue::from_str(&format!("Verification error: {:?}", e)))?;
        // // Combine only the data shards
        let mut full_data = Vec::new();
        for shard in shards.iter().take(2) {
            full_data.extend_from_slice(shard.as_ref().unwrap());
        }
        Ok(full_data)
    }

    // first decrypt key
    // reconstruct shard
    // create signer from shard

    /// Assumes that the personal shard is still encrypted unless given otherwise by passing `project_shard` to false.
    pub fn build_signer(
        &mut self,
        password: String,
        project_shard: Option<bool>,
    ) -> Result<Signer, Error> {
        // build as a form of validation
        self.build()?;
        // check if project_shard arg is false; this means signer is being build with a recovery shard
        if let Some(project_shard) = project_shard {
            if !project_shard {
                if let Some(shard) = self.recovery_shard.take() {
                    let shard = decrypt(&shard, password.as_bytes())
                        .map_err(|e| JsValue::from_str(&format!("Decryption error: {:?}", e)))?;
                    self.recovery_shard = Some(shard);
                } else {
                    return Err(JsValue::from_str(&format!(
                        "Recovery shard does not exist on base Wallet"
                    ))
                    .into());
                }
            }
        } else {
            if let Some(shard) = self.project_shard.take() {
                let shard = decrypt(&shard, password.as_bytes())
                    .map_err(|e| JsValue::from_str(&format!("Decryption error: {:?}", e)))?;
                self.project_shard = Some(shard);
            } else {
                return Err(JsValue::from_str(&format!(
                    "Project shard does not exist on base Wallet"
                ))
                .into());
            }
        }
        let seed = self.reconstruct_shards_internal()?;
        Ok(Signer { key: seed })
    }
}

#[wasm_bindgen]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signer {
    key: Vec<u8>,
}

#[wasm_bindgen]
impl Signer {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Vec<u8>) -> Result<Signer, JsValue> {
        Ok(Signer { key: value })
    }

    #[wasm_bindgen]
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    #[wasm_bindgen]
    pub fn get_public_key(&self) -> String {
        self.generate_root_public_key().to_string(Prefix::XPUB)
    }
    #[wasm_bindgen]
    pub fn verify_root_key(&self, public_key: String) -> bool {
        self.get_public_key() == public_key
    }
}

impl Signer {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.clone()
    }
}

#[wasm_bindgen()]
pub fn new_with_object(value: JsValue) {
    let obj = js_sys::Object::from(value);

    let get_string = |key: &str| -> Result<Option<Vec<u8>>, JsValue> {
        let val = js_sys::Reflect::get(&obj, &JsValue::from_str(key))?;
        if val.is_undefined() || val.is_null() {
            Ok(None)
        } else {
            let s = val
                .as_string()
                .ok_or_else(|| JsValue::from_str(&format!("Field '{key}' must be a string")))?;
            general_purpose::STANDARD
                .decode(&s)
                .map(Some)
                .map_err(|e| JsValue::from_str(&format!("Failed to decode '{key}': {e}")))
        }
    };

    BaseWallet {
        project_shard: get_string("project_shard").unwrap(),
        system_shard: get_string("system_shard").unwrap(),
        recovery_shard: get_string("recovery_shard").unwrap(),
    };
}
