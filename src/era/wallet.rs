use js_sys::{Error, EvalError};

use crate::{wasm_bindgen, JsValue, ReedSolomon};
use crate::{Deserialize, ErasureError, Uint8Array};

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
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Result<BaseWallet, JsValue> {
        serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Failed to deserialize BaseWallet: {}", e)))
    }

    #[wasm_bindgen]
    pub fn reconstruct_shards(&self) -> Result<JsValue, JsValue> {
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
        Ok(Uint8Array::from(full_data.as_slice()).into())
    }
}

impl BaseWallet {
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
}
