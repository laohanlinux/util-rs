use std::fmt;

pub mod serialize;

/// use third package alias local package
pub use hex::{FromHex, FromHexError};

pub trait ToHex {
    fn write_hex<W: fmt::Write> (&self, w: &mut W) -> fmt::Result;
    fn write_hex_upper<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;
}