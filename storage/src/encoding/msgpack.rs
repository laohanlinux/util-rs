use sha3::{Sha3_256, Digest};
use std::string::String;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io::Cursor;

use common;
use crypto::*;

use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};
use rmps::decode::Error;

/////////////////////////////////////////////
#[macro_export]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{self, Write};
    #[test]
    fn u8_hsh() {
        let u_8: u8 = u8::from(100);
        writeln!(io::stdout(), "u8_hash {:?}", u_8.hash()).unwrap();
    }

    #[test]
    fn bool_hash(){
        writeln!(io::stdout(), "bool_true_hash {:?}", true.hash()).unwrap();
        writeln!(io::stdout(), "bool_false_hash {:?}", false.hash()).unwrap();
    }

    #[test]
    fn i8_hash(){
        writeln!(io::stdout(), "i8_hash {:?}", i8::from(100).hash()).unwrap();
    }

    #[test]
    fn batch() {
        for i in 0..(2<<10) {
            writeln!(io::stdout(), "random_{} {:?}",i, i.hash()).unwrap();
        }
    }
}