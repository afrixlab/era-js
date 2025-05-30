mod crypto;
mod era;
mod erasure_coding;
mod key;

pub use crypto::*;
pub use era::*;
pub use erasure_coding::*;
pub use key::*;

//use anyhow::Result;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use hex::*;
use js_sys::Uint8Array;
use reed_solomon_erasure::galois_8::ReedSolomon;

use bip32::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
