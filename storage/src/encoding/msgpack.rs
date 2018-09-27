use std::io::Cursor;
use std::string::String;
use std::time::Duration;

use crypto::*;
use ethkey::Public as PublicKey;
use sha3::{Digest, Sha3_256};

use chrono::prelude::*;
use rmps::decode::Error;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/////////////////////////////////////////////
#[macro_export]
macro_rules! implement_cryptohash_traits {
    ($key: ident) => {
        impl CryptoHash for $key {
            fn hash(&self) -> Hash {
                let mut buf: Vec<u8> = Vec::new();
                self.serialize(&mut Serializer::new(&mut buf)).unwrap();
                hash(&buf)
            }
        }
    };
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
implement_cryptohash_traits! {PublicKey}

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
    use ethkey::Generator;
    use std::io::{self, Write};
    #[test]
    fn u8_hsh() {
        let u_8: u8 = u8::from(100);
        writeln!(io::stdout(), "u8_hash {:?}", u_8.hash()).unwrap();
    }

    #[test]
    fn bool_hash() {
        writeln!(io::stdout(), "bool_true_hash {:?}", true.hash()).unwrap();
        writeln!(io::stdout(), "bool_false_hash {:?}", false.hash()).unwrap();
    }

    #[test]
    fn i8_hash() {
        writeln!(io::stdout(), "i8_hash {:?}", i8::from(100).hash()).unwrap();
    }

    #[test]
    fn publickey_hash() {
        (0..100).for_each(|i| {
            let keypair = ::ethkey::Random {}.generate().unwrap();
            let mut buf = Vec::new();
            keypair
                .public()
                .serialize(&mut Serializer::new(&mut buf))
                .unwrap();
            writeln!(io::stdout(), "{}", buf.len()).unwrap();
        })
    }

    #[test]
    fn vec_hash() {
        let mut buf = Vec::new();
        let v: Vec<u8> = vec![];
        v.serialize(&mut Serializer::new(&mut buf)).unwrap();
        writeln!(io::stdout(), "{}", buf.len()).unwrap();
    }

    #[test]
    fn batch() {
        for i in 0..(2 << 10) {
            writeln!(io::stdout(), "random_{} {:?}", i, i.hash()).unwrap();
        }
    }
}
