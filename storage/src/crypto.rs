use sha3::{Digest, Sha3_256};
use std::io::Cursor;
use std::iter::FromIterator;
use std::string::String;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use common;

use chrono::prelude::*;
use ethereum_types::{Public, Secret, U256};
use rmps::decode::Error;
use rmps::{Deserializer, Serializer};
use rustc_hex::{FromHex, FromHexError, ToHex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const HASH_SIZE: usize = 32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Hash([u8; HASH_SIZE]);

impl Hash {
    // Create a new instance from bytes array.
    pub fn new(b: &[u8]) -> Self {
        assert_eq!(b.len(), HASH_SIZE);
        let mut buf = [0; 32];
        for item in b.iter().enumerate() {
            buf[item.0] = *item.1;
        }
        Hash(buf)
    }

    /// Create a new instance from bytes slice
    pub fn from_slice(bs: &[u8]) -> Option<Self> {
        assert_eq!(bs.len(), HASH_SIZE);
        // TODO
        None
    }

    /// Create a new install with filled with zeros.
    pub fn zero() -> Self {
        Hash([0; HASH_SIZE])
    }

    pub fn to_hex(&self) -> String {
        common::to_hex(self)
    }
}

/// It is very good
impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

use std::str::FromStr;
impl FromStr for Hash {
    type Err = FromHexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == HASH_SIZE {
            let out: Vec<u8> = s.chars().map(|c| c as u8).collect();
            return Ok(Hash::new(&out));
        } else if s.len() == (HASH_SIZE + 2) {
            let out: Vec<u8> = s.chars().skip(2).map(|c| c as u8).collect();
            return Ok(Hash::new(&out));
        } else {
            return Err(FromHexError::InvalidHexLength);
        }
    }
}

use std::fmt;
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}

impl ToHex for Hash {
    fn to_hex<T: FromIterator<char>>(&self) -> T {
        self.as_ref().to_hex()
    }
}

pub trait CryptoHash {
    fn hash(&self) -> Hash;
}

impl Default for Hash {
    fn default() -> Hash {
        Hash::zero()
    }
}

pub fn hash<T: AsRef<[u8]>>(data: T) -> Hash {
    let digest = common::to_keccak(data);
    Hash::new(digest.as_ref())
}

#[derive(Debug, Default)]
pub struct HashStream(Sha3_256);

impl HashStream {
    /// Create a new instance of `HashStream`
    pub fn new() -> Self {
        HashStream(Sha3_256::default())
    }

    /// Processes a chunk of stream and returns a `HashStream` with the updated internal state.
    pub fn update(mut self, chunk: &[u8]) -> Self {
        self.0.input(chunk);
        self
    }

    /// Returns the hash of data supplied to the stream so far.
    pub fn hash(self) -> Hash {
        let dig = self.0.result().to_vec();
        Hash::new(&dig)
    }
}

#[cfg(test)]
mod test {
    use std::io::{self, Write};

    #[test]
    fn hash() {
        for i in 0..100 {
            writeln!(io::stdout(), "{:#?}", super::hash(vec![i])).unwrap();
        }
    }
}
