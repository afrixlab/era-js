#![allow(unused_imports, unused_variables)]
use std::fmt;
use std::fmt::Display;

use super::{to_value, wasm_bindgen, JsValue, ReedSolomon, Uint8Array};

#[derive(Debug)]
pub enum ErasureError {
    FragmentationError,
}

impl std::error::Error for ErasureError {}

impl Into<JsValue> for ErasureError {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

impl Display for ErasureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErasureError::FragmentationError => write!(f, "Fragmentation error"),
        }
    }
}

/// Encodes a byte array into fragments using Reed-Solomon erasure coding.
/// The byte array must be a multiple of the data shards.
///
/// # Arguments
///
/// * `data` - The data to encode
/// * `data_shards` - The number of data shards to encode the data into
/// * `parity_shards` - The number of parity shards to encode the data into
///
/// # Returns
///
/// A Vec of byte arrays containing the encoded data.
#[wasm_bindgen(js_name = encodeShards)]
pub fn encode_shards(
    data: Vec<u8>,
    data_shards: usize,
    parity_shards: usize,
) -> Result<JsValue, JsValue> {
    if data.len() % data_shards != 0 {
        panic!("fragmentation error")
    }
    let reed_solomon = ReedSolomon::new(data_shards, parity_shards).unwrap();
    let size_per_shard = data.len() / data_shards;
    let mut shards = Vec::new();
    for i in 0..data_shards {
        shards.push(data[(size_per_shard * i)..size_per_shard * (i + 1)].to_vec());
    }
    (0..parity_shards).for_each(|_| shards.push(vec![0; size_per_shard]));
    reed_solomon
        .encode(&mut shards)
        .map_err(|_| <ErasureError as Into<JsValue>>::into(ErasureError::FragmentationError))?;
    Ok(to_value(&shards)?)
}

/// Decodes a Vec of byte arrays using Reed-Solomon erasure coding.
/// The alignment of shards in the reconstruction.
/// e.g if shard_1, shard_2, shard_3 is missing.
/// the array should be `[null,null,null,shard_4,shard_5]`
///
/// # Arguments
///
/// * `shards` - The shards to reconstruct
/// * `data_shards` - The number of data shards the shards were encoded into
/// * `parity_shards` - The number of parity shards the shards were encoded into
///
/// # Returns
///
/// A Vec of byte arrays containing the decoded data.
/// If the data could not be decoded, an error is returned.
#[wasm_bindgen(js_name = decodeShards)]
pub fn reconstruct_shards(
    shards: Vec<JsValue>,
    data_shards: usize,
    parity_shards: usize,
) -> Result<JsValue, JsValue> {
    let reed_solomon = ReedSolomon::new(data_shards, parity_shards).unwrap();
    let mut shards: Vec<Option<Vec<u8>>> = shards
        .into_iter()
        .map(|js_value| {
            if js_value.is_null() || js_value.is_undefined() {
                None
            } else {
                Some(Uint8Array::from(js_value).to_vec())
            }
        })
        .collect();
    reed_solomon
        .reconstruct_data(&mut shards)
        .map_err(|_| <ErasureError as Into<JsValue>>::into(ErasureError::FragmentationError))?;
    let shard_refs: Vec<&[u8]> = shards.iter().map(|x| x.as_deref().unwrap()).collect();

    reed_solomon
        .verify(&shard_refs)
        .map_err(|e| JsValue::from_str(&format!("Verification error: {:?}", e)))?;

    // Combine only the data shards
    let mut full_data = Vec::new();
    for shard in shards.iter().take(data_shards) {
        full_data.extend_from_slice(shard.as_ref().unwrap());
    }
    Ok(Uint8Array::from(full_data.as_slice()).into())
}
