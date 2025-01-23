use std::fmt::Display;
use std::fmt;


use super::{ReedSolomon, wasm_bindgen, JsValue, to_value};

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
#[wasm_bindgen(js_name = encodeFragment)]
pub fn encode_fragment(data: Vec<u8>, data_shards: usize, parity_shards: usize) -> Result<JsValue, JsValue> {
     if data.len() % data_shards != 0 {
        panic!("fragmentation error")
    }
    let reed_solomon = ReedSolomon::new(data_shards, parity_shards).unwrap();
    let size_per_shard = data.len() / data_shards;
    let mut shards = Vec::new();
    for i in 0..data_shards{
        shards.push(data[(size_per_shard * i)..size_per_shard * (i + 1)].to_vec());
    }
    (0..parity_shards).for_each(|_| shards.push(vec![0; size_per_shard]));
    reed_solomon.encode(&mut shards).map_err(|_| <ErasureError as Into<JsValue>>::into(ErasureError::FragmentationError))?;
    Ok(to_value(&shards)?)
   
}
