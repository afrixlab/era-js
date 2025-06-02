mod chains;
mod crypto;
mod era;

use crypto::*;
use era::*;
//use chains::*;

//use anyhow::Result;

pub use base64::{engine::general_purpose, Engine};
use bip32::*;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use hex::*;
use js_sys::Uint8Array;
use reed_solomon_erasure::galois_8::ReedSolomon;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
pub use simple_crypt::{decrypt, encrypt};
pub use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use wasm_bindgen::prelude::*;
