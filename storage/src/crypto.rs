use sha3::{Sha3_256, Digest};
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH};

use common;
use encoding::{ToHex, FromHexError};

use uuid::Uuid;
use chrono::{DateTime, Utc};

pub const HASH_SIZE: usize = 32;

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
        Self::new(&[0; HASH_SIZE])
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
            let out:Vec<u8> = s.chars().map(|c| c as u8).collect();
            return Ok(Hash::new(&out));
        }else if  s.len() == (HASH_SIZE + 2)  {
            let out:Vec<u8> = s.chars().skip(2).map(|c| c as u8).collect();
            return Ok(Hash::new(&out));
        } else {
            return Err(FromHexError::new());
        }
    }
}

use std::fmt;
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}

/// TODO use macro
impl ToHex for Hash {
    // TODO
    fn write_hex<W: ::std::fmt::Write>(&self, w: &mut W) -> ::std::fmt::Result {
        Ok(())
    }
    // TODO
    fn write_hex_upper<W: ::std::fmt::Write>(&self, w: &mut W) -> ::std::fmt::Result {
        Ok(())
    }
}

pub trait CryptoHash {
    fn hash(&self) -> Hash;
}

/////////////////////////////////////////////
use std::io::Cursor;

use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};
use rmps::decode::Error;


macro_rules! implement_cryptohash_traits {
    ($key: ident) => {
        impl CryptoHash for $key {
            fn hash(&self) -> Hash {
                let mut buf:Vec<u8> = Vec::new();
                let result = self.serialize(&mut Serializer::new(&mut buf)).unwrap();
                hash(&buf)
            }
        }
    }
}

implement_cryptohash_traits! {bool}
implement_cryptohash_traits! {u8}
implement_cryptohash_traits! {u16}
implement_cryptohash_traits! {u32}
implement_cryptohash_traits! {u64}
implement_cryptohash_traits! {i8}
implement_cryptohash_traits! {i16}
implement_cryptohash_traits! {i32}
implement_cryptohash_traits! {i64}
implement_cryptohash_traits! {String}

impl CryptoHash for () {
    fn hash(&self) -> Hash {
//        let mut buf = Vec::new();
//        hash(self.serialize(&mut Serialize::new(&mut buf)).unwrap())
        Hash::default()
    }
}

impl CryptoHash for Vec<u8> {
    fn hash(&self) -> Hash {
//        let mut buf = Vec::new();
//        hash(self.serialize(&mut Serialize::new(&mut buf)).unwrap())
        Hash::default()
    }
}

impl CryptoHash for Uuid {
    fn hash(&self) -> Hash {
//        let mut buf = Vec::new();
//        hash(self.serialize(&mut Serialize::new(&mut buf)).unwrap())
        Hash::default()
    }
}

impl CryptoHash for DateTime<Utc> {
    fn hash(&self) -> Hash {
        let mut buf = Vec::new();
//        hash(self.serialize(&mut Serialize::new(&mut buf)).unwrap())
        Hash::default()
    }
}

impl Default for Hash {
    fn default() -> Hash {
        Hash::zero()
    }
}

pub fn hash(data: &[u8]) -> Hash {
    let digest = common::to_sha3(data);
    Hash::new(&digest)
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
        let mut hasher = Sha3_256::default();
        hasher.input(data);
        let result = hasher.result();
        Hash::new(&result)
    }
}