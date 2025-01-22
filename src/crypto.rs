use bip32::Prefix;
use crate::Serialize;

use super::{Account, DerivationPath, XPrv, XPub};

#[derive(Debug, Serialize)]
pub struct KeyObject {
    pub private_key: String,
    pub public_key: String,
    pub mnemonic: String,
    pub path: String,
    pub index: u32,
    pub depth: u8,
}

pub trait KeyPath {
    type KeyObject;

    /// Generates the root private key from the seed
    fn generate_root_key(&self) -> XPrv;
    /// Generates the root public key from the seed
    fn generate_root_public_key(&self) -> XPub;
    /// Generates an extended key from the seed
    fn generate_extended_key(&self, path: &str) -> Self::KeyObject;
}

impl KeyPath for Account {
    type KeyObject = KeyObject;

    fn generate_root_key(&self) -> XPrv {
        XPrv::new(&self.to_bytes()).unwrap()
    }

    fn generate_root_public_key(&self) -> XPub {
        let xprv = self.generate_root_key();
        xprv.public_key()
    }

    fn generate_extended_key(&self, path: &str) -> Self::KeyObject {
        let path = <DerivationPath as std::str::FromStr>::from_str(path).unwrap();
        let prefix = Prefix::XPUB;
        let xpriv = XPrv::derive_from_path(self.to_bytes(), &path).unwrap();
        let private_key = format!("0x{}", hex::encode(xpriv.to_bytes()));
        let key_object = KeyObject {
            private_key,
            public_key: xpriv.public_key().to_string(prefix),
            mnemonic: self.to_str(),
            path: path.to_string(),
            index: xpriv.attrs().child_number.index(),
            depth: xpriv.attrs().depth,
        };
        key_object
    }
}
