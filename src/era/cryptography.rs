use crate::{decrypt, wasm_bindgen, JsValue, Uint8Array, encrypt};

#[wasm_bindgen(js_name = decryptShard)]
pub fn decrypt_shards(shards: Vec<u8>, password: String) -> Result<JsValue, JsValue> {
    let decrypted = decrypt(&shards, &password.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("Encryption error: {:?}", e)))?;
    Ok(Uint8Array::from(decrypted.as_slice()).into())
}



#[wasm_bindgen(js_name = encryptShard)]
pub fn encrypt_shards(shards: Vec<u8>, password: String) -> Result<JsValue, JsValue> {
    let encrypted = encrypt(&shards, &password.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("Encryption error: {:?}", e)))?;
    Ok(Uint8Array::from(encrypted.as_slice()).into())
}