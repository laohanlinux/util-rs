#[macro_use]
pub mod msgpack;

use std::fmt;

/// use third package alias local package
pub use hex::{FromHex, FromHexError};