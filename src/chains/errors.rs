use std::fmt::{self, Display};

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum PolkadotError {
    FragmentationError,
}

impl std::error::Error for PolkadotError {}

impl Into<JsValue> for PolkadotError {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

impl Display for PolkadotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolkadotError::FragmentationError => write!(f, "Fragmentation error"),
        }
    }
}
