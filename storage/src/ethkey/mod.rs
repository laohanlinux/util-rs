pub mod secret;
pub mod error;
pub mod crypto;
pub mod random;
pub mod keypair;
pub mod keccak;
pub mod signature;

pub use self::error::Error;
pub use self::secret::Secret;
pub use self::keypair::KeyPair;


#[macro_use]
use encoding::*;

use secp256k1::Secp256k1;
use ethereum_types::H256;
pub use ethereum_types::{Address, Public};

lazy_static! {
    pub static ref SECP256K1: Secp256k1 = Secp256k1::new();
}

/// Uninstantiatable error type for infallible generators.
#[derive(Debug)]
pub enum Void {}

/// Generates new keypair.
pub trait Generator {
    type Error;

    /// Should be called to generate new kaypair
    fn generate(&mut self) -> Result<KeyPair, Self::Error>;
}