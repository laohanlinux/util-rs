use sha3::{Sha3_256, Digest};
use std::string::String;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io::Cursor;

use common;
use encoding::{ToHex, FromHexError};
use crypto::*;

use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};
use rmps::decode::Error;

/////////////////////////////////////////////
macro_rules! implement_cryptohash_traits {
    ($key: ident) => {
        impl CryptoHash for $key {
            fn hash(&self) -> Hash {
                let mut buf:Vec<u8> = Vec::new();
                self.serialize(&mut Serializer::new(&mut buf)).unwrap();
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
implement_cryptohash_traits! {Uuid}
implement_cryptohash_traits! {Duration}

impl CryptoHash for () {
    fn hash(&self) -> Hash {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        hash(&buf)
    }
}

impl CryptoHash for Vec<u8> {
    fn hash(&self) -> Hash {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        hash(&buf)
    }
}

impl CryptoHash for DateTime<Utc> {
    fn hash(&self) -> Hash {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        hash(&buf)
    }
}