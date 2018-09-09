pub mod secret;
pub mod error;

pub use self::error::Error;

use secp256k1::Secp256k1;
use ethereum_types::H256;
pub use ethereum_types::{Address, Public};

lazy_static! {
    pub static ref SECP256K1: Secp256k1 = Secp256k1::new();
}
